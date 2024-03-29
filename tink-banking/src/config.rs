use std::{env, fs, path::Path, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TinkConfig {
    pub id: String,
    pub secret: String,
    pub url: String,
}

static CONFIG: Mutex<Option<TinkConfig>> = Mutex::new(None);

pub fn load_config_from_file<P: AsRef<Path>>(path: P) {
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

pub fn load_config_from_env() {
    let mut config = CONFIG.lock().unwrap();
    if config.is_some() {
        panic!("tink config already loaded");
    }

    let id = env::var("TINK_ID").expect("'TINK_ID' not found in env");
    let secret = env::var("TINK_SECRET").expect("'TINK_SECRET' not found in env");
    let url = env::var("TINK_URL").expect("'TINK_URL' not found in env");

    *config = Some(TinkConfig { id, secret, url });
}

pub fn get_config() -> TinkConfig {
    let config = CONFIG.lock().unwrap();

    config.as_ref().expect("tink config is not loaded").clone()
}

pub fn get_url() -> String {
    get_config().url
}