use clap::Parser;

#[derive(Parser, Debug)]
pub struct PetraArgs {
    /// The port on which to start the server
    pub port: Option<u32>,
    /// The path to the database file
    pub db_path: Option<String>,
    /// The path to the file containing the tink secrets
    pub tink_secret_path: Option<String>,
}

#[derive(Debug)]
pub struct ServerConfig {
    /// The port on which to start the server
    pub port: u32,
    /// The path to the database file
    pub db_path: String,
    /// The path to the file containing the tink secrets
    pub tink_secret_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8187,
            db_path: "data.sqlite".into(),
            tink_secret_path: "tink.secret".into(),
        }
    }
}

impl From<PetraArgs> for ServerConfig {
    fn from(value: PetraArgs) -> Self {
        let default = Self::default();
        Self {
            port: value.port.unwrap_or(default.port),
            db_path: value.db_path.unwrap_or(default.db_path),
            tink_secret_path: value.tink_secret_path.unwrap_or(default.tink_secret_path),
        }
    }
}
