use std::{
    fs::{read_to_string, File},
    io::Write,
    sync::Mutex, path::Path,
};

static TINK_SECRETS: Mutex<Option<(String, String)>> = Mutex::new(None);

pub fn load_tink_secrets<P: AsRef<Path>>(path: P) {
    let secret = TINK_SECRETS.lock().unwrap();
    if secret.is_some() {
        return;
    }
    drop(secret);

    let result = read_to_string(&path);

    match result {
        Ok(content) => parse_secrets(content),
        Err(_) => {
            let mut file = File::create(&path).expect("Could not create tink.secret file");
            file.write(b"CLIENT_ID =\nCLIENT_SECRET=").expect("Could not write to tink.secret file");
            panic!("no tink.secret file found, created it")
        }
    }

    println!("Successfully read tink.secret file");
}

fn parse_secrets(content: String) {
    let mut id: Option<String> = None;
    let mut secret: Option<String> = None;
    for line in content.lines() {
        let parts: Vec<_> = line.split('=').collect();

        let name = parts
            .get(0)
            .expect(&format!("Expected name in line '{}'", line))
            .trim();
        let value = parts
            .get(1)
            .expect(&format!("Expected name in line '{}'", line))
            .trim()
            .to_string();

        match name {
            "CLIENT_ID" => id = Some(value),
            "CLIENT_SECRET" => secret = Some(value),
            _ => panic!("unknown value in tink.secrets '{}'", name),
        }
    }

    let id = id.expect("no CLIENT_ID provided in tink.secret");
    let secret = secret.expect("no CLIENT_SCERET provided in tink.secret");

    *TINK_SECRETS.lock().unwrap() = Some((id, secret));
}

pub fn get_tink_client_id() -> String {
    let secrets = TINK_SECRETS.lock().unwrap();

    let id = secrets.as_ref().expect("tink secrets not loaded yet").0.to_owned();

    id
}

pub fn get_tink_client_secret() -> String {
    let secrets = TINK_SECRETS.lock().unwrap();

    let secret = secrets.as_ref().expect("tink secrets not loaded yet").1.to_owned();

    secret
}
