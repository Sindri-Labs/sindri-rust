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
use openapi::models::CircuitInfoResponse;
use openapi::apis::authorization_api::apikey_list;
use openapi::models::ApiKeyResponse;
use openapi::apis::authorization_api::ApikeyListError;  
use crate::custom_middleware::HeaderDeduplicatorMiddleware;
use crate::custom_middleware::LoggingMiddleware;
use crate::custom_middleware::retry_client;
use reqwest_retry::policies::ExponentialBackoffTimed;



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

        let client = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Could not build client")
        )
        .with(HeaderDeduplicatorMiddleware)
        .with(LoggingMiddleware)
        .with(retry_client::<ExponentialBackoffTimed>(None))
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
            bearer_access_token: api_key,
            client,
            ..Default::default()
        };

        Self {
            config
        }
    }

    pub async fn list_api_keys(&self) -> Result<Vec<ApiKeyResponse>, Error<ApikeyListError>> {
        let api_keys = apikey_list(&self.config).await?;
        Ok(api_keys)
    }   

    pub async fn get_circuit(&self, circuit_id: &str) -> Result<CircuitInfoResponse, Error<CircuitDetailError>> {
        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        Ok(circuit_info)
    }

    // pub async fn prove_circuit(
    //     &self,
    //     circuit_id: &str,
    //     proof_input: &str,
    //     verify: bool,
    //     include_smart_contract_calldata: bool,
    //     meta: serde_json::Value,
    // ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {

    //     #[derive(Serialize)]
    //     struct ProofRequest<'a> {
    //         proof_input: &'a str,
    //         perform_verify: bool,
    //         meta: serde_json::Value,
    //     }

    //     let request_body = ProofRequest {
    //         proof_input,
    //         perform_verify: verify,
    //         meta,
    //     };

    //     let response = self.client
    //         .post(&url)
    //         .bearer_auth(self.api_key.as_ref().ok_or("API key not set")?)
    //         .json(&request_body)
    //         .send()
    //         .await?;

    //     if !response.status().is_success() {
    //         return Err(format!("API request failed: {}", response.status()).into());
    //     }

    //     // Handle polling for proof completion
    //     // Implementation details for polling...

    //     todo!("Implement full proof generation logic")
    // }


}
