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
    }

    outputs = {self, nixpkgs, crane, rust-overlay, flake-utils, ...}:
        flake-utils.lib.eachDefaultSystem
    (system:
        let
            pkgs = import nixpkgs {
                inherit system;
                overlays = [ (import rust-overlays) ];
            };

            rustTarget = pkgs.rust-bin.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
                targets = [ "wasm32-unknown-unknown" ];
            };

            craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;
            src = ./.;

            commonArgs = {
                inherit src;
                buildInputs = [
                    cargo-leptos-git
                    # Other shit
                ];
            };

            cargoArtifacts = craneLib.buildDepsOnly {
                inherit src;
            };

            petra-clippy = craneLib.cargoClippy {
                inherit cargoArtifacts src;
                cargoClippyExtraArgs = "-- --deny warnings";
            }

            petra = craneLib.buildPackage {
                inherit cargoArtifacts src;
                pname = "petra";

                buildPhaseCargoCommand = "cargo leptos build --release -vvv";
                installPhaseCommand = ''
                mkdir -p $out/bin
                cp target/server/x86_64-unknown-linux-gnu/release/petra $out/bin/
                cp -r target/site $out/bin
                '';
                doCheck = false;
            };
        in
        {
            checks = { inherit petra petra-clippy };
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
            #                     }
            #                 }
            #             }
            #         }
            #     }
        }
    )
}