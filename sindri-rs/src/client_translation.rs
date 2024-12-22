use reqwest::{Client as ReqwestClient, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tokio::fs;
use walkdir::WalkDir;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;
use std::io::Write;

// Re-export types
pub use crate::api::{
    CircuitType,
    JobStatus,
    BoojumCircuitInfoResponse,
    CircomCircuitInfoResponse,
    GnarkCircuitInfoResponse,
    Halo2CircuitInfoResponse,
    JoltCircuitInfoResponse,
    NoirCircuitInfoResponse,
    Plonky2CircuitInfoResponse,
    ProofInfoResponse,
    CircuitStatusResponse,
};

#[derive(Debug, Clone)]
pub struct AuthOptions {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RetryOptions {
    pub min_timeout: Duration,
    pub retries: u32,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            min_timeout: Duration::from_secs(1),
            retries: 6,
        }
    }
}

#[derive(Debug)]
pub struct SindriClient {
    client: ReqwestClient,
    api_key: Option<String>,
    base_url: String,
    polling_interval: Duration,
    retry_options: RetryOptions,
}

impl SindriClient {
    pub fn new(auth_options: Option<AuthOptions>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Sindri-Client",
            HeaderValue::from_str(&format!("sindri-rust-sdk/{}", env!("CARGO_PKG_VERSION")))
                .unwrap(),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let auth = auth_options.unwrap_or_default();
        let base_url = auth.base_url
            .or_else(|| std::env::var("SINDRI_BASE_URL").ok())
            .unwrap_or_else(|| "https://sindri.app".to_string());

        let api_key = auth.api_key
            .or_else(|| std::env::var("SINDRI_API_KEY").ok());

        Self {
            client,
            api_key,
            base_url,
            polling_interval: Duration::from_secs(1),
            retry_options: RetryOptions::default(),
        }
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn create_circuit<P: AsRef<Path>>(
        &self,
        project: P,
        tags: Vec<String>,
        meta: serde_json::Value,
    ) -> Result<CircuitInfoResponse, Box<dyn std::error::Error>> {
        let project_path = project.as_ref();
        
        // Validate project path exists
        if !project_path.exists() {
            return Err("Project path does not exist".into());
        }

        // Create multipart form
        let form = reqwest::multipart::Form::new();

        // Add tags
        let tags = if tags.is_empty() {
            vec!["latest".to_string()]
        } else {
            tags
        };

        for tag in tags {
            if !tag.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
                return Err("Invalid tag format".into());
            }
        }

        // Handle project files
        if project_path.is_file() {
            // Handle single file (tarball)
            let file_content = fs::read(project_path).await?;
            // Add file to form
            // Implementation details for adding file to form...
        } else if project_path.is_dir() {
            // Handle directory
            // Create tarball of directory contents
            let mut archive = Builder::new(Vec::new());
            
            for entry in WalkDir::new(project_path)
                .into_iter()
                .filter_entry(|e| !e.file_name().to_str().map_or(false, |s| s.starts_with("."))) 
            {
                let entry = entry?;
                if entry.file_type().is_file() {
                    // Add file to archive
                    // Implementation details for adding to archive...
                }
            }
        }

        // Make API request and handle response
        // Implementation details for API request...

        todo!("Implement full circuit creation logic")
    }

    pub async fn get_circuit(&self, circuit_id: &str) -> Result<CircuitInfoResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/circuit/{}", self.base_url, circuit_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(self.api_key.as_ref().ok_or("API key not set")?)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }

        let circuit_info = response.json().await?;
        Ok(circuit_info)
    }

    pub async fn prove_circuit(
        &self,
        circuit_id: &str,
        proof_input: &str,
        verify: bool,
        include_smart_contract_calldata: bool,
        meta: serde_json::Value,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/circuit/{}/prove", self.base_url, circuit_id);

        #[derive(Serialize)]
        struct ProofRequest<'a> {
            proof_input: &'a str,
            perform_verify: bool,
            meta: serde_json::Value,
        }

        let request_body = ProofRequest {
            proof_input,
            perform_verify: verify,
            meta,
        };

        let response = self.client
            .post(&url)
            .bearer_auth(self.api_key.as_ref().ok_or("API key not set")?)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }

        // Handle polling for proof completion
        // Implementation details for polling...

        todo!("Implement full proof generation logic")
    }
}
