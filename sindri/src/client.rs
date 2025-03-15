//! # The primary module for interacting with Sindri's API.

use std::{collections::HashMap, fs, fs::File, io::Write, path::Path, time::Duration};

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use sindri_openapi::{
    apis::{
        circuit_download, circuit_status,
        circuits_api::{
            circuit_create, circuit_delete, circuit_detail, proof_create, CircuitDetailError,
        },
        configuration::Configuration,
        proof_status,
        proofs_api::{proof_delete, proof_detail, ProofDetailError},
        Error,
    },
    models::{CircuitInfoResponse, CircuitProveInput, JobStatus, ProofInfoResponse},
};
use tracing::{debug, info, warn};

use crate::{
    custom_middleware::{
        retry_client, HeaderDeduplicatorMiddleware, LoggingMiddleware,
        ZstdRequestCompressionMiddleware,
    },
    types::{CircuitInfo, ProofInput},
    utils::compress_directory,
};

#[cfg(any(feature = "record", feature = "replay"))]
use crate::custom_middleware::vcr_middleware;

#[cfg(feature = "rich-terminal")]
use console::style;
#[cfg(feature = "rich-terminal")]
use indicatif::{ProgressBar, ProgressStyle};

/// Configuration options for authenticating with the Sindri API.
///
/// This struct is used to configure authentication when initializing a [`SindriClient`].
/// While these options can be passed directly in code, it is generally recommended to
/// set them using environment variables instead (`SINDRI_API_KEY` and `SINDRI_BASE_URL`).
///
/// # Fields
///
/// * `api_key` - Optional API key for authentication. If not provided, falls back to `SINDRI_API_KEY` environment variable
/// * `base_url` - Optional base URL for API requests. Should be left as `None` except for internal development purposes.
///                If not provided, falls back to `SINDRI_BASE_URL` environment variable, then to the default production URL
///
/// # Examples
///
/// ```
/// use sindri::client::AuthOptions;
///
/// // Explicitly passing API key within code
/// let auth = AuthOptions {
///     api_key: Some("my_api_key".to_string()),
///     base_url: None, // Use default production URL
/// };
/// ```
#[derive(Default, Debug, Clone)]
pub struct AuthOptions {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

/// Configuration options for polling behavior when waiting for long-running operations.
///
/// This struct is used to configure how a [`SindriClient`] polls Sindri's API while
/// waiting for operations like circuit compilation or proof generation to complete.
/// It can be provided during client initialization or modified afterwards through
/// the client's `polling_options` field.
///
/// # Fields
///
/// * `interval` - Duration to wait between API status checks (default: 1 second)
/// * `timeout` - Optional maximum duration to wait for operation completion (default: 10 minutes)
///
/// # Examples
///
/// ```
/// use sindri::client::{SindriClient, PollingOptions};
/// use std::time::Duration;
///
/// // Create client with custom polling options
/// let polling = PollingOptions {
///     interval: Duration::from_secs(5),
///     timeout: Some(Duration::from_secs(1800)), // 30 minutes
/// };
/// let client = SindriClient::new(None, Some(polling));
///
/// // Or modify polling options after creation
/// let mut client = SindriClient::new(None, None);
/// client.polling_options.timeout = Some(Duration::from_secs(1800));
/// ```
///
/// The default values are suitable for most use cases and only need to be adjusted
/// when working with circuits or proofs that are known to require longer processing times.
#[derive(Debug, Clone)]
pub struct PollingOptions {
    pub interval: Duration,
    pub timeout: Option<Duration>,
}
impl Default for PollingOptions {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            timeout: Some(Duration::from_secs(60 * 10)),
        }
    }
}

/// The [`SindriClient`] struct encapsulates all the necessary methods and properties
///  required to communicate effectively with the Sindri API, handling tasks
///  like uploads of circuits or guest code and proof generation.
#[derive(Debug)]
pub struct SindriClient {
    config: Configuration,
    pub polling_options: PollingOptions,
}

impl Default for SindriClient {
    /// Creates a new Sindri API client with default options.
    ///
    /// This is equivalent to calling `SindriClient::new(None, None)`.
    /// Authentication will be read from environment variables and default polling options will be used.
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl SindriClient {
    /// Creates a new Sindri API client.
    ///
    /// # Arguments
    ///
    /// * `auth_options` - Optional authentication configuration. If not provided, will attempt to read from environment variables
    /// * `polling_options` - Optional polling configuration for long-running operations
    ///
    /// # Environment Variables
    ///
    /// * `SINDRI_API_KEY` - API key for authentication (if auth_options not provided)
    /// * `SINDRI_BASE_URL` - Base URL for API requests (if auth_options not provided)
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    /// let client = SindriClient::new(None, None); // inferring your API key from `SINDRI_API_KEY`
    /// ```
    pub fn new(auth_options: Option<AuthOptions>, polling_options: Option<PollingOptions>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Sindri-Client",
            HeaderValue::from_str(
                format!("{}/v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
            )
            .expect("Could not insert default rust client header"),
        );

        #[allow(unused_mut)] // needed for VCR mutation
        let mut client_builder = reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .zstd(true)
                .build()
                .expect("Could not build client"),
        )
        .with(HeaderDeduplicatorMiddleware)
        .with(LoggingMiddleware)
        .with(retry_client(None))
        .with(ZstdRequestCompressionMiddleware);

        #[cfg(any(feature = "record", feature = "replay"))]
        {
            // Do not apply vcr to unit tests
            if !cfg!(test) {
                let bundle = std::env::var("VCR_PATH")
                    .unwrap_or_else(|_| "tests/recordings/replay.vcr.json".to_string());
                let bundle_path = std::path::PathBuf::from(&bundle);

                #[cfg(feature = "replay")]
                if !bundle_path.exists() {
                    panic!("Recording not found at: {}", bundle_path.display());
                }

                client_builder = client_builder.with(vcr_middleware(bundle_path));
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

        Self {
            config,
            polling_options: polling_options.unwrap_or_default(),
        }
    }

    /// Returns the configured API key
    pub fn api_key(&self) -> Option<&str> {
        self.config.bearer_access_token.as_deref()
    }

    /// Returns the configured base URL for API requests
    pub fn base_url(&self) -> &str {
        &self.config.base_path
    }

    /// Sets the API key for this client.
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default()
    ///     .with_api_key("my_api_key");
    /// ```
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.config.bearer_access_token = Some(api_key.into());
        self
    }

    /// Sets the base URL for this client.
    ///
    /// Should be left as default except for internal development purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default()
    ///     .with_base_url("https://custom.sindri.app");
    /// ```
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.config.base_path = base_url.into();
        self
    }

    /// Sets the polling interval for this client.
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    /// use std::time::Duration;
    ///
    /// let client = SindriClient::default()
    ///     .with_polling_interval(Duration::from_secs(5));
    /// ```
    pub fn with_polling_interval(mut self, interval: Duration) -> Self {
        self.polling_options.interval = interval;
        self
    }

    /// Sets the polling timeout for this client.
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    /// use std::time::Duration;
    ///
    /// let client = SindriClient::default()
    ///     .with_timeout(Duration::from_secs(1800)); // 30 minutes
    /// ```
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.polling_options.timeout = Some(timeout);
        self
    }

    /// Removes the polling timeout for this client.
    ///
    /// # Examples
    ///
    /// ```
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default()
    ///     .with_no_timeout();
    /// ```
    pub fn with_no_timeout(mut self) -> Self {
        self.polling_options.timeout = None;
        self
    }

    /// Creates and deploys a new circuit from a local project.
    ///
    /// In order to generate proofs on Sindri, you must first deploy the zero-knowledge circuit or
    /// guest code with this method. Upon deployment, this method continuously polls the service to
    /// track the compilation status until the process either completes successfully or fails.
    ///
    /// # Arguments
    ///
    /// * `project` - Path to a local project directory or an archive file (.zip, .tar, .tar.gz, .tgz)
    /// * `tags` - Optional list of tags to identify the circuit
    /// * `meta` - Optional metadata (key-value pairs) to associate with the circuit
    ///
    /// # Returns
    ///
    /// Returns circuit information on successful compilation, or error if compilation fails or times out.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use std::collections::HashMap;
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default();
    /// let project = "path/to/directory/or/tarfile".to_string();
    /// let tags: Option<Vec<String>> = Some(vec!["a_custom_tag".to_string()]);
    /// let meta: Option<HashMap<String, String>> = Some(HashMap::from([("key".to_string(), "value".to_string())]));
    /// let circuit = client.create_circuit(
    ///     project,
    ///     tags,
    ///     meta
    /// ).await.unwrap();
    /// # });
    /// ```
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
        #[cfg(feature = "rich-terminal")]
        println!(
            "{}",
            style(format!(
                "  ✓ Valid tags specified: {}",
                tags.as_ref().map_or(0, |t| t.len())
            ))
            .cyan()
        );

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
                #[cfg(feature = "rich-terminal")]
                println!("{}", style("  ✓ Detected compressed project file").cyan());
                fs::read(&project)?
            }
            _ => return Err("Project is not a file or directory".into()),
        };

        info!("Uploading circuit to Sindri");
        #[cfg(feature = "rich-terminal")]
        println!("{}", style("Uploading circuit...").bold());

        #[cfg(feature = "rich-terminal")]
        let pb = ProgressBar::new_spinner();
        #[cfg(feature = "rich-terminal")]
        pb.enable_steady_tick(Duration::from_millis(120));
        #[cfg(feature = "rich-terminal")]
        pb.set_style(
            ProgressStyle::with_template("{spinner} {msg:.cyan}")
                .unwrap()
                .tick_strings(&crate::utils::CLOCK_TICKS),
        );
        #[cfg(feature = "rich-terminal")]
        pb.set_message("Sending files to circuit create endpoint...");

        let response = circuit_create(&self.config, project_bytes, meta, tags).await?;

        let circuit_id = response.id();
        info!("Circuit created with ID: {}", circuit_id);

        #[cfg(feature = "rich-terminal")]
        let mut current_status = *response.status();
        #[cfg(feature = "rich-terminal")]
        pb.set_message(format!("Job status: {}", current_status));

        let start_time = std::time::Instant::now();
        let mut status = circuit_status(&self.config, circuit_id).await?;
        debug!("Initial circuit status: {:?}", status.status);

        while !matches!(status.status, JobStatus::Ready | JobStatus::Failed) {
            if let Some(timeout) = self.polling_options.timeout {
                if start_time.elapsed() > timeout {
                    warn!("Circuit compilation timed out after {:?}", timeout);
                    return Err(
                        "Circuit compilation did not complete within timeout duration".into(),
                    );
                }
            }
            std::thread::sleep(self.polling_options.interval);
            status = circuit_status(&self.config, circuit_id).await?;
            #[cfg(feature = "rich-terminal")]
            if status.status != current_status {
                pb.set_message(format!("Job status: {}", status.status));
                current_status = status.status;
            }
        }

        match status.status {
            JobStatus::Ready => info!(
                "Circuit compilation completed successfully after {:?}",
                start_time.elapsed()
            ),
            JobStatus::Failed => warn!(
                "Circuit compilation failed after {:?}",
                start_time.elapsed()
            ),
            _ => unreachable!(),
        }

        let circuit_info = circuit_detail(&self.config, circuit_id, None).await?;
        Ok(circuit_info)
    }

    /// Blocking version of `create_circuit`.
    ///
    /// This method provides the same functionality as `create_circuit` but can be used
    /// in synchronous contexts. It internally creates a runtime to execute the async operation.
    pub fn create_circuit_blocking(
        &self,
        project: String,
        tags: Option<Vec<String>>,
        meta: Option<HashMap<String, String>>,
    ) -> Result<CircuitInfoResponse, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(self.create_circuit(project, tags, meta))
    }

    /// Deletes a circuit by ID.
    ///
    /// # Arguments
    ///
    /// * `circuit_id` - ID of the circuit to delete
    ///
    /// # Warning
    ///
    /// Once deleted, the circuit will no longer be viewable on the Sindri dashboard
    /// and you will not be able to generate proofs from it. You should only delete a circuit
    /// if its existence may cause confusion or misuse.
    pub async fn delete_circuit(&self, circuit_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting circuit with ID: {}", circuit_id);
        circuit_delete(&self.config, circuit_id).await?;
        Ok(())
    }

    /// Downloads and saves a project's source code.
    ///
    /// This method allows you to retrieve the original source code that was uploaded to Sindri for a given circuit.
    /// The code is downloaded as a tarball (.tar.gz) and saved to the specified location. This is useful for:
    ///
    /// - Obtaining code from public circuits to use as examples or templates
    /// - Retrieving your own previously deployed circuits
    /// - Backing up circuit source code
    /// - Collaborating by sharing circuit implementations
    ///
    /// # Arguments
    ///
    /// * `circuit_id` - ID of the circuit to clone
    /// * `download_path` - Path where the circuit archive should be saved
    /// * `override_max_project_size` - Optional maximum allowed size in bytes (defaults to 2GB)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default();
    ///
    /// let project_build_id = "team_name/project_name:tag";
    /// let download_path = "path/to/save/circuit.tar.gz".to_string();
    ///
    /// client.clone_circuit(
    ///     project_build_id,
    ///     download_path,
    ///     None
    /// ).await.unwrap();
    /// # });
    /// ```
    pub async fn clone_circuit(
        &self,
        circuit_id: &str,
        download_path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cloning circuit with ID: {}", circuit_id);
        debug!("Download path: {}", download_path);

        let download_response = circuit_download(&self.config, circuit_id, None).await?;
        debug!("Circuit downloaded successfully");
        // Write the circuit to the specified path
        let mut file = File::create(download_path.clone())?;
        file.write_all(&download_response.bytes().await?)?;
        info!("Circuit written to {}", download_path);
        Ok(())
    }

    /// Blocking version of `clone_circuit`.
    ///
    /// This method provides the same functionality as `clone_circuit` but can be used
    /// in synchronous contexts. It internally creates a runtime to execute the async operation.
    pub fn clone_circuit_blocking(
        &self,
        circuit_id: &str,
        download_path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(self.clone_circuit(circuit_id, download_path))
    }

    /// Retrieves the details of a circuit.
    ///
    /// You can use this method to get the status, metadata, and other details of a circuit.
    ///
    /// # Arguments
    ///
    /// * `circuit_id` - ID of the circuit to retrieve
    /// * `include_verification_key` - Whether to include the verification key in the response
    ///
    ///  # Returns
    ///
    /// A [`CircuitInfoResponse`] object containing the circuit details.  
    /// The enum variant corresponds to the type of circuit deployed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default();
    /// let project_build_id = "team_name/project_name:tag";
    /// let circuit = client.get_circuit(project_build_id, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_circuit(
        &self,
        circuit_id: &str,
        include_verification_key: Option<bool>,
    ) -> Result<CircuitInfoResponse, Error<CircuitDetailError>> {
        info!("Getting circuit with ID: {}", circuit_id);
        let circuit_info =
            circuit_detail(&self.config, circuit_id, include_verification_key).await?;
        Ok(circuit_info)
    }

    /// Creates and generates a proof for a circuit.
    ///
    /// This method initiates proof generation and automatically polls the Sindri API until the proof
    /// is either successfully generated or fails. The polling interval and timeout can be configured
    /// through the client's `polling_options`.
    ///
    /// # Arguments
    ///
    /// * `circuit_id` - ID of the circuit to prove
    /// * `proof_input` - Input values for the proof. Can be provided as a JSON object, &str, or String.
    ///                   The format (JSON, TOML, base64, etc.) should match your circuit's expected input structure.
    /// * `meta` - Optional metadata key-value pairs
    /// * `verify` - Whether to verify the proof (server-side) after generation. A proof status
    ///              of `Failed` would be returned if the proof is not valid.
    /// * `prover_implementation` - Optional specific prover implementation to use.
    ///                            This field is generally for internal development only.
    ///                            Sindri automatically selects the most performant implementation
    ///                            based on your project's deployment details.
    ///
    /// # Returns
    ///
    /// Returns proof information on successful generation, or error if generation fails or times out.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default();
    /// let project_build_id = "team_name/project_name:tag";
    /// let proof_input = "x=10,y=20";
    /// let proof = client.prove_circuit(project_build_id, proof_input, None, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn prove_circuit(
        &self,
        circuit_id: &str,
        proof_input: impl Into<ProofInput>,
        meta: Option<HashMap<String, String>>,
        verify: Option<bool>,
        prover_implementation: Option<String>,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {
        info!("Creating proof for circuit: {}", circuit_id);
        debug!(
            "Proof metadata: {:?}, verify: {:?}, prover: {:?}",
            meta, verify, prover_implementation
        );

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
            JobStatus::Ready => info!(
                "Proof generation completed successfully after {:?}",
                start_time.elapsed()
            ),
            JobStatus::Failed => warn!("Proof generation failed after {:?}", start_time.elapsed()),
            _ => unreachable!(),
        }

        let proof_info = proof_detail(&self.config, &proof_id, None, None, None, None).await?;
        Ok(proof_info)
    }

    /// Blocking version of `prove_circuit`.
    ///
    /// This method provides the same functionality as `prove_circuit` but can be used
    /// in synchronous contexts. It internally creates a runtime to execute the async operation.
    pub fn prove_circuit_blocking(
        &self,
        circuit_id: &str,
        proof_input: impl Into<ProofInput>,
        meta: Option<HashMap<String, String>>,
        verify: Option<bool>,
        prover_implementation: Option<String>,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(self.prove_circuit(
            circuit_id,
            proof_input,
            meta,
            verify,
            prover_implementation,
        ))
    }

    /// Deletes a proof by ID.
    ///
    /// # Arguments
    ///
    /// * `proof_id` - ID of the proof to delete
    ///
    /// # Warning
    ///
    /// Once deleted, the proof will no longer be viewable on the Sindri dashboard.
    /// You should only delete a proof if its existence may cause confusion and retrieval
    /// of the wrong proof details.
    pub async fn delete_proof(&self, proof_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting proof with ID: {}", proof_id);
        proof_delete(&self.config, proof_id).await?;
        Ok(())
    }

    /// Gets information about a proof.
    ///
    /// This method allows you to retrieve details about a previously generated proof, including
    /// the proof data itself and any public outputs. While `prove_circuit()` returns
    /// the same information immediately after generation, this method is particularly useful for:
    ///
    /// - Retrieving details of historical proofs
    /// - Fetching public outputs from previous proof generations
    /// - Verifying proof status after system interruptions
    /// - Downloading proof data for external verification
    ///
    /// # Arguments
    ///
    /// * `proof_id` - ID of the proof to retrieve
    /// * `include_proof` - Whether to include the proof data in the response
    /// * `include_public` - Whether to include public inputs/outputs in the response
    /// * `include_verification_key` - Whether to include the verification key in the response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use sindri::client::SindriClient;
    ///
    /// let client = SindriClient::default();
    /// let proof_id = "uuid-assigned-during-proof-generation";
    ///
    /// // Get just the proof status
    /// let basic_info = client.get_proof(proof_id, None, None, None).await.unwrap();
    ///
    /// // Get just the public outputs
    /// let proof_with_outputs = client.get_proof(proof_id, None, Some(true), None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_proof(
        &self,
        proof_id: &str,
        include_proof: Option<bool>,
        include_public: Option<bool>,
        include_verification_key: Option<bool>,
    ) -> Result<ProofInfoResponse, Error<ProofDetailError>> {
        let proof_info = proof_detail(
            &self.config,
            proof_id,
            include_proof,
            include_public,
            None,
            include_verification_key,
        )
        .await?;
        Ok(proof_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BoojumCircuitInfoResponse;
    use tracing_test::traced_test;
    use wiremock::{
        matchers::{header_exists, method, path},
        Mock, MockServer, ResponseTemplate,
    };

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
    fn test_builder_methods() {
        // Test chaining all builder methods
        let client = SindriClient::default()
            .with_api_key("test_key")
            .with_base_url("https://example.com")
            .with_polling_interval(Duration::from_secs(5))
            .with_timeout(Duration::from_secs(300));

        // Verify all settings were applied correctly
        assert_eq!(client.api_key(), Some("test_key"));
        assert_eq!(client.base_url(), "https://example.com");
        assert_eq!(client.polling_options.interval, Duration::from_secs(5));
        assert_eq!(
            client.polling_options.timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_with_no_timeout() {
        // Test that with_no_timeout removes the timeout
        let client = SindriClient::default()
            .with_timeout(Duration::from_secs(300))
            .with_no_timeout();

        assert_eq!(client.polling_options.timeout, None);
    }

    #[test]
    fn test_new_client_with_env_vars() {
        temp_env::with_vars(
            vec![
                ("SINDRI_API_KEY", Some("env_test_key")),
                ("SINDRI_BASE_URL", Some("https://example.com")),
            ],
            || {
                let client = SindriClient::new(None, None);
                assert_eq!(client.api_key(), Some("env_test_key"));
                assert_eq!(client.base_url(), "https://example.com");
            },
        );
    }

    #[test]
    fn test_auth_options_override_env_vars() {
        temp_env::with_vars(
            vec![
                ("SINDRI_API_KEY", Some("env_test_key")),
                ("SINDRI_BASE_URL", Some("https://example.com")),
            ],
            || {
                let auth_options = AuthOptions {
                    api_key: Some("test_key".to_string()),
                    base_url: Some("https://other.example.com".to_string()),
                };
                let client = SindriClient::new(Some(auth_options), None);
                // authoptions should override env vars
                assert_eq!(client.api_key(), Some("test_key"));
                assert_eq!(client.base_url(), "https://other.example.com");
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
        assert_eq!(
            client.polling_options.timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_post_client_init_polling_tweaks() {
        let mut client = SindriClient::new(None, None);

        assert_eq!(client.polling_options.interval, Duration::from_secs(1));
        assert_eq!(
            client.polling_options.timeout,
            Some(Duration::from_secs(600))
        );

        client.polling_options.interval = Duration::from_secs(5);
        client.polling_options.timeout = Some(Duration::from_secs(300));

        assert_eq!(client.polling_options.interval, Duration::from_secs(5));
        assert_eq!(
            client.polling_options.timeout,
            Some(Duration::from_secs(300))
        );
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
            .respond_with(
                ResponseTemplate::new(200).set_body_json(CircuitInfoResponse::Boojum(Box::new(
                    BoojumCircuitInfoResponse {
                        circuit_id: "test_circuit_123".to_string(),
                        ..Default::default()
                    },
                ))),
            )
            .mount(&mock_server)
            .await;

        wiremock::Mock::given(method("GET"))
            .and(path("/api/v1/circuit/test_circuit_123/status"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(CircuitInfoResponse::Boojum(Box::new(
                    BoojumCircuitInfoResponse {
                        status: JobStatus::Ready,
                        ..Default::default()
                    },
                ))),
            )
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
        let _result = client
            .create_circuit(test_file.to_str().unwrap().to_string(), None, None)
            .await;

        // Create method logs (debug + info level)
        assert!(logs_contain("Creating new circuit from project"));
        assert!(logs_contain("Uploading circuit to Sindri"));
        assert!(logs_contain("Circuit created with ID: test_circuit_123"));
        assert!(logs_contain("Circuit compilation completed"));
        // Logs from the request/response (debug level)
        // Ensure we see exactly three (create, status, detail)
        logs_assert(|lines: &[&str]| {
            match lines
                .iter()
                .filter(|line| line.contains("Request sent"))
                .count()
            {
                3 => Ok(()),
                n => Err(format!(
                    "Expected three logs for request outbound, but found {}",
                    n
                )),
            }
        });
        logs_assert(|lines: &[&str]| {
            match lines
                .iter()
                .filter(|line| line.contains("Response received"))
                .count()
            {
                3 => Ok(()),
                n => Err(format!(
                    "Expected three logs for response inbound, but found {}",
                    n
                )),
            }
        });
    }
}
