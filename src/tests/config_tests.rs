// Create a local Config struct for testing
struct Config {
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        Self { environment }
    }
}
use std::env;

#[test]
fn test_config_from_env() {
    env::set_var("APP_ENV", "testing");
    let config = Config::from_env();
    assert_eq!(config.environment, "testing");
}
