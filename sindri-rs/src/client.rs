use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::Part,
    Client, Response, StatusCode,
};
use reqwest_middleware::ClientWithMiddleware;
use openapi::apis::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct AuthOptions {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}
impl Default for AuthOptions {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
        }
    }
}

#[derive(Debug)]
pub struct SindriClient {
    config: Configuration,
}

impl SindriClient {
    pub fn new(auth_options: Option<AuthOptions>) -> Self {

        let mut headers = HeaderMap::new();
        headers.insert(
            "Sindri-Client",
            HeaderValue::from_str(&"sindri-rust-sdk/alpha")
                .expect("Could not insert default rust client header"),
        );

        // Default (no retry for now)
        let client = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Could not build client")
        )
        .build();

        // First try to read from auth_options, then from environment variables, then use default values    
        let auth = auth_options.unwrap_or_default();
        let base_url = auth.base_url
            .or_else(|| std::env::var("SINDRI_BASE_URL").ok())
            .unwrap_or_else(|| "https://sindri.app".to_string());
        let api_key = auth.api_key
            .or_else(|| std::env::var("SINDRI_API_KEY").ok());

        let config = Configuration {
            base_path: base_url,
            client,
            bearer_access_token: api_key,
            ..Default::default()
        };

        Self {
            config
        }
    }
}
