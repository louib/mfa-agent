use std::env;

pub const DEFAULT_DB_FILE_NAME: &str = "mfa-agent.kdbx";

pub fn get_db_file_path() -> String {
    // TODO use the config for the db path?
    let db_dir = match env::home_dir() {
        Some(p) => p.display().to_string(),
        None => ".".to_string(),
    };
    db_dir + "/" + DEFAULT_DB_FILE_NAME
}

pub mod Database {
    pub fn get_totp_secrets() -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}
