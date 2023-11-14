{
    description = "Money Managing webserver";

    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";

        crane = {
            url = "github:ipetkov/crane";
            inputs.nixpkgs.follows = "nixpkgs";
        };

        cargo-leptos-git.url = "github:leptos-rs/cargo-leptos";

        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };

        flake-utils = {
            url = "github:numtide/flake-utils";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = {self, nixpkgs, crane, rust-overlay, flake-utils, cargo-leptos-git, ...}:
        flake-utils.lib.eachDefaultSystem
    (system:
        let
            pkgs = import nixpkgs {
                inherit system;
                overlays = [ (import rust-overlay) ];
            };

            inherit (pkgs) lib;

            rustTarget = pkgs.rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
                targets = [ "wasm32-unknown-unknown" ];
            };

            craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;
            src = ./.;

            commonArgs = {
                inherit src;
                buildInputs = [
                    cargo-leptos
                    pkgs.pkg-config
                    pkgs.openssl
                    pkgs.protobuf
                    pkgs.binaryen
                    pkgs.cargo-generate
                    pkgs.dart-sass
                ] ++ lib.optionals pkgs.stdenv.isDarwin [
                    pkgs.libiconv
                ];
            };

            cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
                cargoExtraArgs = " --target x86_64-unknown-linux-gnu";
                doCheck = false;

                cargoVendorDir = craneLib.vendorMultipleCargoDeps {
                    inherit (craneLib.findCargoFiles src) cargoConfigs;
                    cargoLockList = [
                        ./Cargo.lock
                    ];
                };
            });

            petra-clippy = craneLib.cargoClippy (commonArgs // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "-- --deny warnings";
                doCheck = false;
            });

            # ---------- CRANE BUILD HERE ----------

            petra = craneLib.buildPackage (commonArgs // {
                inherit cargoArtifacts;
                pname = "petra";

                buildPhaseCargoCommand = "cargo leptos build --release -vvv";
                installPhaseCommand = ''
                mkdir -p $out/bin
                cp target/release/petra $out/bin/
                cp -r target/site $out/bin
                '';
                doCheck = false;

                # LEPTOS_BIN_TARGET_TRIPLE = "x86_64-unknown-linux-gnu";
                # LEPTOS_BIN_PROFILE_RELEASE = "release";
                # LEPTOS_LIB_PROFILE_RELEASE = "release-wasm-size";
            });

            # ---------- CARGO LEPTOS BUILD HERE ----------

            cargo-leptos = pkgs.rustPlatform.buildRustPackage rec {
                pname = "cargo-leptos";
                version = "0.1.11";
                buildFeatures = ["no_downloads"];

                src = cargo-leptos-git;

                cargoSha256 = "sha256-U9iPgVjdqSkHLOY6efIE8CPUoesT2heCTaM18Y/UztE=";

                nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];

                buildInputs = with pkgs; [openssl pkg-config] ++ lib.optionals stdenv.isDarwin [ Security ];

                doCheck = false;

                meta = with lib; {
                    description = "Leptos build tool";
                };
            };
        in
        {
            checks = { inherit petra petra-clippy; };
            packages.default = petra;
            # nixosModules.default = { config, lib, pkgs, ...}:
            #     with lib;
            #     let cfg = config.brndel.services.petra;
            #     in
            #     {
            #         options.brndel.services.petra = {
            #             enable = mkEnableOption "Enable petra webserver service";

            #             config = mkIf cfg.enable {
            #                 systemd.services.petra = {
            #                     description = "Petra service";
            #                     wantedBy = [ "multi-user.target" ];

            #                     serviceConfig = {
            #                         DynamicUser = "yes";
            #                         ExecStart = "${cfg.package}/bin/petra";
            #                         Restart = "on-failure";
            #                         RestartSec = "5s";
            #                     };
            #                 };
            #             };
            #         };
            #     };
        }
    );
}