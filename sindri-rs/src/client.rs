use std::{collections::HashMap, fs, fs::File, io::Write, path::Path, time::Duration};

use openapi::{
    apis::{
        circuit_download, circuit_status, proof_status,
        circuits_api::{circuit_create, circuit_delete, circuit_detail, proof_create, CircuitDetailError},
        configuration::Configuration,
        proofs_api::{proof_delete, proof_detail, ProofDetailError},
        Error,
    },
    models::{CircuitInfoResponse, CircuitProveInput, JobStatus, ProofInfoResponse},
};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_retry::policies::ExponentialBackoffTimed;
use tracing::{info, debug, warn};

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

#[derive(Debug, Clone)]
pub struct PollingOptions {
    pub interval: Duration,
    pub timeout: Option<Duration>,
}
impl Default for PollingOptions {
    fn default() -> Self {
        Self { interval: Duration::from_secs(1), timeout: Some(Duration::from_secs(60 * 10)) }
    }
}


#[derive(Debug)]
pub struct SindriClient {
    config: Configuration,
    pub polling_options: PollingOptions,
}

impl SindriClient {
    pub fn new(auth_options: Option<AuthOptions>, polling_options: Option<PollingOptions>) -> Self {

        let mut headers = HeaderMap::new();
        headers.insert(
            "Sindri-Client",
            HeaderValue::from_str(format!("{}/v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str())
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

        Self { config, polling_options: polling_options.unwrap_or_default() }
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
        info!("Creating new circuit from project: {}", project);
        debug!("Circuit tags: {:?}, metadata: {:?}", tags, meta);

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
            p if p.is_dir() => {
                info!("Compressing directory for upload");
                compress_directory(p, None).await?
            }
            p if p.is_file() => {
                let extension_regex = Regex::new(r"(?i)\.(zip|tar|tar\.gz|tgz)$")?;
                if !extension_regex.is_match(&project) {
                    return Err("Project is not a zip file or tarball".into());
                }
                fs::read(&project)?
            }
            _ => return Err("Project is not a file or directory".into()),
        };

        info!("Uploading circuit to Sindri");
        let response = circuit_create(&self.config, project_bytes, meta, tags).await?;

        let circuit_id = response.id();
        info!("Circuit created with ID: {}", circuit_id);

        let mut status = circuit_status(&self.config, circuit_id).await?;
        debug!("Initial circuit status: {:?}", status.status);

        let start_time = std::time::Instant::now();
        while !matches!(status.status, JobStatus::Ready | JobStatus::Failed) {
            if let Some(timeout) = self.polling_options.timeout {
                if start_time.elapsed() > timeout {
                    warn!("Circuit compilation timed out after {:?}", timeout);
                    return Err("Circuit compilation did not complete within timeout duration".into());
                }
            }
            std::thread::sleep(self.polling_options.interval);
            status = circuit_status(&self.config, circuit_id).await?;
        }

        match status.status {
            JobStatus::Ready => info!("Circuit compilation completed successfully after {:?}", start_time.elapsed()),
            JobStatus::Failed => warn!("Circuit compilation failed after {:?}", start_time.elapsed()),
            _ => unreachable!(),
        }

        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        Ok(circuit_info)
    }

    pub async fn delete_circuit(&self, circuit_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting circuit with ID: {}", circuit_id);
        circuit_delete(&self.config, circuit_id).await?;
        Ok(())
    }

    pub async fn clone_circuit(&self, circuit_id: &str, download_path: String, override_max_project_size: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cloning circuit with ID: {}", circuit_id);
        debug!("Download path: {}", download_path);
        // Ensure circuit exists, is ready, and is not too large
        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        if *circuit_info.status() != JobStatus::Ready {
            warn!("Circuit does not indicate ready status");
            return Err("Circuit does not indicate ready status".into());
        }
        if circuit_info.file_size().unwrap_or(0) as u64 > override_max_project_size.unwrap_or(2 * 1024 * 1024 * 1024) as u64 { // 2GB
            warn!("Circuit tarball is larger than {} bytes.", override_max_project_size.unwrap_or(2* 1024 * 1024 * 1024));
            return Err(format!("Circuit tarball is larger than {} bytes. If you still want to clone it, pass a larger size into `override_max_project_size`", override_max_project_size.unwrap_or(2* 1024 * 1024 * 1024)).into());
        }

        // Then, download the circuit
        let download_response = circuit_download(&self.config, circuit_id, None).await?;
        debug!("Circuit downloaded successfully");
        // Write the circuit to the specified path
        let mut file = File::create(download_path.clone())?;
        file.write_all(&download_response.bytes().await?)?;
        info!("Circuit written to {}", download_path);
        Ok(())
    }

    pub async fn get_circuit(
        &self,
        circuit_id: &str,
        include_verification_key: Option<bool>,
    ) -> Result<CircuitInfoResponse, Error<CircuitDetailError>> {
        info!("Getting circuit with ID: {}", circuit_id);
        let circuit_info = circuit_detail(&self.config, circuit_id, include_verification_key).await?;
        Ok(circuit_info)
    }

    pub async fn prove_circuit(
        &self,
        circuit_id: &str,
        proof_input: impl Into<ProofInput>,
        meta: Option<HashMap<String, String>>,
        verify: Option<bool>,
        prover_implementation: Option<String>,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {
        info!("Creating proof for circuit: {}", circuit_id);
        debug!("Proof metadata: {:?}, verify: {:?}, prover: {:?}", meta, verify, prover_implementation);

        let circuit_prove_input = CircuitProveInput {
            proof_input: Box::new(proof_input.into().0),
            perform_verify: verify,
            meta,
            prover_implementation,
        };

        let proof_info = proof_create(&self.config, circuit_id, circuit_prove_input).await?;
        let proof_id = proof_info.proof_id;
        info!("Proof generation started with ID: {}", proof_id);

        let mut status = proof_status(&self.config, &proof_id).await?;
        debug!("Initial proof status: {:?}", status.status);

        let start_time = std::time::Instant::now();
        while !matches!(status.status, JobStatus::Ready | JobStatus::Failed) {
            if let Some(timeout) = self.polling_options.timeout {
                if start_time.elapsed() > timeout {
                    warn!("Proof generation timed out after {:?}", timeout);
                    return Err("Proof generation did not complete within timeout duration".into());
                }
            }
            std::thread::sleep(self.polling_options.interval);
            status = proof_status(&self.config, &proof_id).await?;
        }

        match status.status {
            JobStatus::Ready => info!("Proof generation completed successfully after {:?}", start_time.elapsed()),
            JobStatus::Failed => warn!("Proof generation failed after {:?}", start_time.elapsed()),
            _ => unreachable!(),
        }

        let proof_info = proof_detail(&self.config, &proof_id, None, None, None, None).await?;
        Ok(proof_info)
    }

    pub async fn delete_proof(&self, proof_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting proof with ID: {}", proof_id);
        proof_delete(&self.config, proof_id).await?;
        Ok(())
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
    use tracing_test::traced_test;
    use wiremock::{
        matchers::{header_exists, method, path},
        Mock, MockServer, ResponseTemplate,
    };
    use crate::BoojumCircuitInfoResponse;

    #[test]
    fn test_new_client_with_options() {
        let auth_options = AuthOptions {
            api_key: Some("test_key".to_string()),
            base_url: Some("https://fake.sindri.app".to_string()),
        };
        let client = SindriClient::new(Some(auth_options), None);

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
                let client = SindriClient::new(None, None);
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
                let client = SindriClient::new(Some(auth_options), None);
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
                let client = SindriClient::new(None, None);
                assert_eq!(client.api_key(), None);
                assert_eq!(client.base_url(), "https://sindri.app");
            },
        );
    }

    #[test]
    fn test_new_client_config_defaults() {
        let client = SindriClient::new(None, None);
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

    #[test]
    fn test_polling_options_custom() {
        let polling_options = PollingOptions {
            interval: Duration::from_secs(5),
            timeout: Some(Duration::from_secs(300)), // 5 minutes
        };
        let client = SindriClient::new(None, Some(polling_options));
        
        assert_eq!(client.polling_options.interval, Duration::from_secs(5));
        assert_eq!(client.polling_options.timeout, Some(Duration::from_secs(300)));
    }

    #[test]
    fn test_post_client_init_polling_tweaks() {
        let mut client = SindriClient::new(None, None);
        
        assert_eq!(client.polling_options.interval, Duration::from_secs(1));
        assert_eq!(client.polling_options.timeout, Some(Duration::from_secs(600))); 

        client.polling_options.interval = Duration::from_secs(5);
        client.polling_options.timeout = Some(Duration::from_secs(300));

        assert_eq!(client.polling_options.interval, Duration::from_secs(5));
        assert_eq!(client.polling_options.timeout, Some(Duration::from_secs(300)));
    }

    #[tokio::test]
    async fn test_client_default_header() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header_exists("sindri-client"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let outer_client = SindriClient::new(None, None);
        let inner_client = &outer_client.config.client;

        let request = inner_client.get(mock_server.uri()).build().unwrap();
        let response = inner_client.execute(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_circuit_create_tag_validation() {
        let client = SindriClient::new(None, None);

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

    async fn mock_compile_server() -> MockServer {
        // Setup mock server
        let mock_server = wiremock::MockServer::start().await;
    
        // Setup mock responses
        wiremock::Mock::given(method("POST"))
            .and(path("/api/v1/circuit/create"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(CircuitInfoResponse::Boojum(Box::new(BoojumCircuitInfoResponse {
                    circuit_id: "test_circuit_123".to_string(),
                    ..Default::default()
                }))))
            .mount(&mock_server)
            .await;
    
        wiremock::Mock::given(method("GET"))
            .and(path("/api/v1/circuit/test_circuit_123/status"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(CircuitInfoResponse::Boojum(Box::new(BoojumCircuitInfoResponse {
                    status: JobStatus::Ready,
                    ..Default::default()
                }))))
            .mount(&mock_server)
            .await;
    
        // The circuit detail is returned from the method, but does not influence logging
        wiremock::Mock::given(method("GET"))
            .and(path("/api/v1/circuit/test_circuit_123/detail"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        mock_server
    }

    #[tokio::test]
    #[traced_test]
    async fn test_verbose_logging() {

        let mock_server = mock_compile_server().await;

        // Create client with test configuration
        let auth_options = AuthOptions {
            api_key: Some("test_key".to_string()),
            base_url: Some(mock_server.uri()),
        };
        let client = SindriClient::new(Some(auth_options), None);
    
        // Create a temporary test directory
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.zip");
        std::fs::write(&test_file, "test content").unwrap();
    
        // Execute the operation
        let _result = client.create_circuit(
            test_file.to_str().unwrap().to_string(),
            None,
            None
        ).await;

        // Create method logs (debug + info level)
        assert!(logs_contain("Creating new circuit from project"));
        assert!(logs_contain("Uploading circuit to Sindri"));
        assert!(logs_contain("Circuit created with ID: test_circuit_123"));
        assert!(logs_contain("Circuit compilation completed"));
        // Logs from the request/response (debug level)
        // Ensure we see exactly three (create, status, detail)
        logs_assert(|lines: &[&str]| {
            match lines.iter().filter(|line| line.contains("Request sent")).count() {
                3 => Ok(()),
                n => Err(format!("Expected three logs for request outbound, but found {}", n)),
            }
        });
        logs_assert(|lines: &[&str]| {
            match lines.iter().filter(|line| line.contains("Response received")).count() {
                3 => Ok(()),
                n => Err(format!("Expected three logs for response inbound, but found {}", n)),
            }
        });
    }

}
