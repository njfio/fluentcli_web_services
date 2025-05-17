use std::env;

pub struct Config {
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        Self { environment }
    }
}
