use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::Part,
    Client, Response, StatusCode,
};
use reqwest_middleware::ClientWithMiddleware;
use openapi::apis::configuration::Configuration;
use openapi::apis::circuits_api::circuit_detail;
use openapi::apis::Error;
use openapi::apis::circuits_api::CircuitDetailError;
use openapi::apis::authorization_api::apikey_list;
use openapi::apis::authorization_api::ApikeyListError;  
use reqwest_tracing::TracingMiddleware;

use crate::utils::HeaderDeduplicatorMiddleware;




#[derive(Default, Debug, Clone)]
pub struct AuthOptions {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
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
            HeaderValue::from_str(&"DOESTHISWORK???")
                .expect("Could not insert default rust client header"),
        );

        // Default (no retry for now)
        let client = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Could not build client")
        )
        .with(TracingMiddleware::default())
        .with(HeaderDeduplicatorMiddleware)
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

    pub async fn list_api_keys(&self) -> Result<serde_json::Value, Error<ApikeyListError>> {
        let api_keys = apikey_list(&self.config).await?;
        Ok(api_keys)
    }   

    pub async fn get_circuit(&self, circuit_id: &str) -> Result<serde_json::Value, Error<CircuitDetailError>> {
        let circuit_info = circuit_detail(&self.config, circuit_id.into(), None).await?;
        Ok(circuit_info)
    }


}
