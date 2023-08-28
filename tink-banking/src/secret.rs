use std::{path::Path, fs, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TinkConfig {
    pub id: String,
    pub secret: String,
}

static CONFIG: Mutex<Option<TinkConfig>> = Mutex::new(None);

pub fn load_secret_from_file<P: AsRef<Path>>(path: P) {
    let mut config = CONFIG.lock().unwrap();
    if config.is_some() {
        panic!("tink config already loaded");
    }

    let content = fs::read_to_string(path).expect("could not read tink secret file");

    match toml::from_str(&content) {
        Ok(cfg) => *config = cfg,
        Err(err) => panic!("could not parse tink file '{}'", err),
    }
}

pub fn get_config() -> TinkConfig {
    let config = CONFIG.lock().unwrap();

    config.as_ref().expect("tink config is not loaded").clone()
}