use crate::client::{AuthOptions, SindriClient};

#[test]
fn test_new_client_with_options() {
    let auth_options = AuthOptions {
        api_key: Some("test_key".to_string()),
        base_url: Some("https://dev.sindri.app".to_string()),
    };
    let client = SindriClient::new(Some(auth_options));

    assert_eq!(client.api_key(), Some("test_key"));
    assert_eq!(client.base_url(), "https://dev.sindri.app");
}

#[test]
fn test_new_client_with_env_vars() {
    temp_env::with_vars(
        vec![
            ("SINDRI_API_KEY", Some("env_test_key")),
            ("SINDRI_BASE_URL", Some("https://dev.sindri.app")),
        ],
        || {
            let client = SindriClient::new(None);
            assert_eq!(client.api_key(), Some("env_test_key"));
            assert_eq!(client.base_url(), "https://dev.sindri.app");
        },
    );
}

#[test]
fn test_auth_options_override_env_vars() {
    temp_env::with_vars(
        vec![
            ("SINDRI_API_KEY", Some("env_test_key")),
            ("SINDRI_BASE_URL", Some("https://dev.sindri.app")),
        ],
        || {
            let auth_options = AuthOptions {
                api_key: Some("test_key".to_string()),
                base_url: Some("https://other.sindri.app".to_string()),
            };
            let client = SindriClient::new(Some(auth_options));
            // authoptions should override env vars
            assert_eq!(client.api_key(), Some("test_key"));
            assert_eq!(client.base_url(), "https://other.sindri.app");
        },
    );
}

#[test]
fn test_new_client_auth_defaults() {
    temp_env::with_vars(
        vec![
            ("SINDRI_API_KEY", None::<String>),
            ("SINDRI_BASE_URL", None::<String>),
        ],
        || {
            let client = SindriClient::new(None);
            assert_eq!(client.api_key(), None);
            assert_eq!(client.base_url(), "https://sindri.app");
        },
    );
}

#[test]
fn test_new_client_config_defaults() {
    let client = SindriClient::new(None);
    let config = client.config();
    // Ensure the fields we are not setting have not changed between openapi codegen
    assert_eq!(
        config.user_agent,
        Some("OpenAPI-Generator/v1.14.5/rust".to_owned())
    );
    assert_eq!(config.basic_auth, None);
    assert_eq!(config.oauth_access_token, None);
    // the api_key field in the config struct is unused and an unfortunate name overlap
    // `bearer_access_token` is the actual config field that handles Sindri API keys
    assert!(config.api_key.is_none());
}
