//! These methods will submit a project deploy or proof request to Sindri,
//! without waiting for the job to complete.

use std::{collections::HashMap, fs, path::Path};

use regex::Regex;
use sindri_openapi::{
    apis::circuits_api::{circuit_create, proof_create},
    models::{CircuitInfoResponse, CircuitProveInput, ProofInfoResponse},
};
use tracing::{debug, info};

use crate::{client::SindriClient, types::ProofInput, utils::compress_directory};

#[cfg(feature = "rich-terminal")]
use crate::utils::ClockProgressBar;
#[cfg(feature = "rich-terminal")]
use console::style;

impl SindriClient {
    /// Deploys a new circuit from a local project (without waiting for job completion).
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
    /// Returns circuit identifier on successful request.
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
    /// let circuit_response = client.request_build(
    ///     project,
    ///     tags,
    ///     meta
    /// ).await.unwrap();
    /// # });
    /// ```
    pub async fn request_build(
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
        let pb = ClockProgressBar::new("Sending files to circuit create endpoint...");

        let response = circuit_create(&self.config, project_bytes, meta, tags).await?;

        #[cfg(feature = "rich-terminal")]
        pb.clear();

        Ok(response)
    }

    /// Requests proof generation for a circuit (without waiting for job completion).
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
    /// Returns proof identifier on successful request.
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
    /// let proof_response = client.request_proof(project_build_id, proof_input, None, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn request_proof(
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

        Ok(proof_info)
    }
}
