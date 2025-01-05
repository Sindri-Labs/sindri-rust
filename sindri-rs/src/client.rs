use std::{collections::HashMap, fs, path::Path};

use openapi::{
    apis::{
        circuit_status,
        circuits_api::{circuit_create, circuit_detail, proof_create, CircuitDetailError},
        configuration::Configuration,
        proof_status,
        proofs_api::{proof_detail, ProofDetailError},
        Error,
    },
    models::{CircuitInfoResponse, CircuitProveInput, JobStatus, ProofInfoResponse},
};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_retry::policies::ExponentialBackoffTimed;

use crate::{
    custom_middleware::{retry_client, HeaderDeduplicatorMiddleware, LoggingMiddleware},
    types::{CircuitInfo, ProofInput},
    utils::compress_directory,
};

#[cfg(any(feature = "record", feature = "replay"))]
use crate::custom_middleware::vcr_middleware;

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
            HeaderValue::from_str("DOESTHISWORK???")
                .expect("Could not insert default rust client header"),
        );

        #[allow(unused_mut)] // needed for VCR mutation
        let mut client_builder = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Could not build client"),
        )
        .with(HeaderDeduplicatorMiddleware)
        .with(LoggingMiddleware)
        .with(retry_client::<ExponentialBackoffTimed>(None));

        #[cfg(any(feature = "record", feature = "replay"))]
        {
            if !cfg!(test) {
                // Do not apply to unit tests
                client_builder = client_builder.with(vcr_middleware());
            }
        }

        let client = client_builder.build();

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

    /// Uploads a circuit project to Sindri. Upon successful upload, the method polls
    /// to track the compilation status of the project.
    /// Returns a CircuitInfoResponse object with the circuit id and other metadata.
    pub async fn create_circuit(
        &self,
        project: String,
        tags: Option<Vec<String>>,
        meta: Option<HashMap<String, String>>,
    ) -> Result<CircuitInfoResponse, Box<dyn std::error::Error>> {
        // Validate tags if provided
        let tag_rules = Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
        if let Some(ref tags) = tags {
            for tag in tags {
                if !tag_rules.is_match(tag) {
                    return Err(format!("\"{tag}\" is not a valid tag. Tags may only contain alphanumeric characters, underscores, hyphens, and periods.").into());
                }
            }
        }

        // Load the project into a byte array whether it is a compressed
        // file already or a directory
        let project_bytes = match Path::new(&project) {
            p if p.is_dir() => compress_directory(p, None).await?,
            p if p.is_file() => {
                let extension_regex = Regex::new(r"(?i)\.(zip|tar|tar\.gz|tgz)$")?;
                if !extension_regex.is_match(&project) {
                    return Err("Project is not a zip file or tarball".into());
                }
                fs::read(&project)?
            }
            _ => return Err("Project is not a file or directory".into()),
        };

        let response = circuit_create(&self.config, project_bytes, meta, tags).await?;

        // openapi returns a union type for the circuit_info response, so we need to match on the specific type
        let circuit_id = response.id();
        let mut status = circuit_status(&self.config, circuit_id).await?;

        // TODO: Implement an optional timeout & configurable polling interval
        while !matches!(status.status, JobStatus::Ready | JobStatus::Failed) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            status = circuit_status(&self.config, circuit_id).await?;
        }

        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        Ok(circuit_info)
    }

    pub async fn get_circuit(
        &self,
        circuit_id: &str,
        include_verification_key: Option<bool>,
    ) -> Result<CircuitInfoResponse, Error<CircuitDetailError>> {
        let circuit_info = circuit_detail(&self.config, circuit_id, include_verification_key).await?;
        Ok(circuit_info)
    }

    pub async fn prove_circuit(
        &self,
        circuit_id: &str,
        proof_input: &str,  
        meta: Option<HashMap<String, String>>,
        verify: Option<bool>,
        prover_implementation: Option<String>,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {

        let proof_input_coerced = Box::new(serde_json::from_str(proof_input)?);

        let circuit_prove_input = CircuitProveInput {
            proof_input: proof_input_coerced,
            perform_verify: verify,
            meta,
            prover_implementation,
        };
        println!("{:?}", circuit_prove_input.proof_input);
        let proof_info = proof_create(&self.config, circuit_id, circuit_prove_input).await?;
        let proof_id = proof_info.proof_id;
        let mut status = proof_status(&self.config, &proof_id).await?;

        // TODO: Implement an optional timeout & configurable polling interval
        while !matches!(status.status, JobStatus::Ready | JobStatus::Failed) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            status = proof_status(&self.config, &proof_id).await?;
        }

        let proof_info = proof_detail(&self.config, &proof_id, None, None, None, None).await?;
        Ok(proof_info)
    }

    pub async fn get_proof(
        &self,
        proof_id: &str,
        include_proof: Option<bool>,
        include_public: Option<bool>,
        include_verification_key: Option<bool>,
    ) -> Result<ProofInfoResponse, Error<ProofDetailError>> {
        let proof_info = proof_detail(&self.config, proof_id, include_proof, include_public, None, include_verification_key).await?;
        Ok(proof_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{header_exists, method},
        Mock, MockServer, ResponseTemplate,
    };

    #[test]
    fn test_new_client_with_options() {
        let auth_options = AuthOptions {
            api_key: Some("test_key".to_string()),
            base_url: Some("https://fake.sindri.app".to_string()),
        };
        let client = SindriClient::new(Some(auth_options));

        assert_eq!(client.api_key(), Some("test_key"));
        assert_eq!(client.base_url(), "https://fake.sindri.app");
    }

    #[test]
    fn test_new_client_with_env_vars() {
        temp_env::with_vars(
            vec![
                ("SINDRI_API_KEY", Some("env_test_key")),
                ("SINDRI_BASE_URL", Some("https://fake.sindri.app")),
            ],
            || {
                let client = SindriClient::new(None);
                assert_eq!(client.api_key(), Some("env_test_key"));
                assert_eq!(client.base_url(), "https://fake.sindri.app");
            },
        );
    }

    #[test]
    fn test_auth_options_override_env_vars() {
        temp_env::with_vars(
            vec![
                ("SINDRI_API_KEY", Some("env_test_key")),
                ("SINDRI_BASE_URL", Some("https://fake.sindri.app")),
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
        let config = client.config;
        // Ensure the fields we are not setting have not changed between openapi codegen
        assert_eq!(
            config.user_agent,
            Some("OpenAPI-Generator/v1.15.1/rust".to_owned())
        );
        assert_eq!(config.basic_auth, None);
        assert_eq!(config.oauth_access_token, None);
        // the api_key field in the config struct is unused and an unfortunate name overlap
        // `bearer_access_token` is the actual config field that handles Sindri API keys
        assert!(config.api_key.is_none());
    }

    #[tokio::test]
    async fn test_client_default_header() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header_exists("sindri-client"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let outer_client = SindriClient::new(None);
        let inner_client = &outer_client.config.client;

        let request = inner_client.get(mock_server.uri()).build().unwrap();
        let response = inner_client.execute(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_circuit_create_tag_validation() {
        let client = SindriClient::new(None);

        let mut tags = vec!["test_t@g".to_string()];
        let mut circuit = client
            .create_circuit("fake_path".to_string(), Some(tags), None)
            .await;
        assert!(circuit.is_err());
        assert!(circuit.unwrap_err().to_string().contains("not a valid tag"));

        tags = vec![
            "test_tag".to_string(),
            "1-2-3-4-5-6".to_string(),
            "ಠ_ಠ".to_string(),
        ];
        circuit = client
            .create_circuit("fake_path".to_string(), Some(tags), None)
            .await;
        assert!(circuit.is_err());
        assert!(circuit.unwrap_err().to_string().contains("ಠ_ಠ"));
    }
}
