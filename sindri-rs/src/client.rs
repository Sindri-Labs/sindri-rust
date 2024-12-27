use openapi::apis::authorization_api::apikey_list;
use openapi::apis::authorization_api::ApikeyListError;
use openapi::apis::circuits_api::circuit_detail;
use openapi::apis::circuits_api::CircuitDetailError;
use openapi::apis::configuration::Configuration;
use openapi::apis::Error;
use openapi::models::ApiKeyResponse;
use openapi::models::CircuitInfoResponse;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::Part,
    Client, Response, StatusCode,
};
use reqwest_middleware::ClientWithMiddleware;

use openapi::apis::circuits_api::proof_create;
use openapi::apis::circuits_api::ProofCreateError;
use openapi::models::CircuitProveInput;
use openapi::models::ProofInfoResponse;
use openapi::models::ProofInput;

use crate::custom_middleware::retry_client;
use crate::custom_middleware::HeaderDeduplicatorMiddleware;
use crate::custom_middleware::LoggingMiddleware;
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
                .expect("Could not build client"),
        )
        .with(HeaderDeduplicatorMiddleware)
        .with(LoggingMiddleware)
        .with(retry_client::<ExponentialBackoffTimed>(None))
        .build();

        // First try to read from auth_options, then from environment variables, then use default values
        let auth = auth_options.unwrap_or_default();
        let base_url = auth
            .base_url
            .or_else(|| std::env::var("SINDRI_BASE_URL").ok())
            .unwrap_or_else(|| "https://sindri.app".to_string());
        let api_key = auth
            .api_key
            .or_else(|| std::env::var("SINDRI_API_KEY").ok());

        let config = Configuration {
            base_path: base_url,
            bearer_access_token: api_key,
            client,
            ..Default::default()
        };

        Self { config }
    }

    pub fn api_key(&self) -> Option<&str> {
        self.config.bearer_access_token.as_deref()
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_path
    }

    pub(crate) fn config(&self) -> &Configuration {
        &self.config
    }

    pub async fn get_circuit(
        &self,
        circuit_id: &str,
    ) -> Result<CircuitInfoResponse, Error<CircuitDetailError>> {
        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        Ok(circuit_info)
    }

    pub async fn prove_circuit(
        &self,
        circuit_id: &str,
        proof_input: &str,
        verify: Option<bool>,
        meta: Option<serde_json::Value>,
    ) -> Result<ProofInfoResponse, Error<ProofCreateError>> {
        let proof_input_coerced = Box::new(serde_json::from_str(proof_input)?);

        let circuit_prove_input = CircuitProveInput {
            proof_input: proof_input_coerced,
            perform_verify: None,        //todo
            meta: None,                  //todo
            prover_implementation: None, //todo
        };

        let proof_info = proof_create(&self.config, circuit_id, circuit_prove_input).await?;
        // TODO: Implement polling for proof completion
        Ok(proof_info)
    }
}
