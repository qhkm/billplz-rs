use std::io::Write;

#[test]
fn test_config_from_env_vars() {
    std::env::set_var("BILLPLZ_API_KEY", "env-test-key");
    std::env::set_var("BILLPLZ_ENVIRONMENT", "staging");

    let config = billplz::cli::config::Config::load(None).unwrap();
    assert_eq!(config.api_key, "env-test-key");
    assert_eq!(config.environment, "staging");

    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");
}

#[test]
fn test_config_from_file() {
    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    let mut f = std::fs::File::create(&config_path).unwrap();
    writeln!(f, r#"api_key = "file-test-key""#).unwrap();
    writeln!(f, r#"environment = "production""#).unwrap();

    let config = billplz::cli::config::Config::load(Some(&config_path)).unwrap();
    assert_eq!(config.api_key, "file-test-key");
    assert_eq!(config.environment, "production");
}

#[test]
fn test_env_vars_override_config_file() {
    std::env::set_var("BILLPLZ_API_KEY", "env-key");
    std::env::set_var("BILLPLZ_ENVIRONMENT", "staging");

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    let mut f = std::fs::File::create(&config_path).unwrap();
    writeln!(f, r#"api_key = "file-key""#).unwrap();
    writeln!(f, r#"environment = "production""#).unwrap();

    let config = billplz::cli::config::Config::load(Some(&config_path)).unwrap();
    assert_eq!(config.api_key, "env-key");
    assert_eq!(config.environment, "staging");

    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");
}

#[test]
fn test_config_missing_api_key_errors() {
    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");

    let result = billplz::cli::config::Config::load(Some(std::path::Path::new("/nonexistent")));
    assert!(result.is_err());
}
