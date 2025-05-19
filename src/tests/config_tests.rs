use crate::config::Config;
use std::env;

#[test]
fn test_config_from_env() {
    env::set_var("APP_ENV", "testing");
    let config = Config::from_env();
    assert_eq!(config.environment, "testing");
}
