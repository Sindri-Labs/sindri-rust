use clap::{command, Parser, Subcommand};
use sindri::client::{AuthOptions, SindriClient};

use sindri_cli::commands::{clone, deploy};

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Cargo {
    Sindri(Cli),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sindri API key (overrides SINDRI_API_KEY env var)
    #[arg(long, global = true)]
    api_key: Option<String>,

    /// Sindri API base URL (overrides SINDRI_BASE_URL env var)
    #[arg(long, global = true)]
    base_url: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clone a circuit
    Clone {
        /// Circuit ID to clone, could be a UUID or a team/project:tag identifier
        #[arg(required = true)]
        circuit: String,

        /// Directory where the circuit should be downloaded
        #[arg(long)]
        directory: Option<String>,
    },
    /// Deploy a circuit
    Deploy {
        /// Path to circuit project directory or archive
        #[arg(required = true)]
        project: String,

        /// Optional tags to identify the circuit (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Optional metadata key-value pairs (format: key=value,key2=value2)
        #[arg(long, value_delimiter = ',')]
        meta: Option<Vec<String>>,
    },
}

fn main() {
    let Cargo::Sindri(args) = Cargo::parse();

    // Initialize client with provided auth options
    let auth = AuthOptions {
        api_key: args.api_key,
        base_url: args.base_url,
    };
    let client = SindriClient::new(Some(auth), None);

    match args.command {
        Commands::Clone { circuit, directory } => {
            clone(&client, circuit, directory);
        }
        Commands::Deploy {
            project,
            tags,
            meta,
        } => {
            deploy(&client, project, tags, meta);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use tempfile::TempDir;
    use wiremock::{
        matchers::{method, path},
        ResponseTemplate,
    };

    #[tokio::test]
    async fn test_cli_deploy_unauthorized() {
        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();

        let circuit_path = "tests/factory/circuit.tar.gz";

        cmd.env("SINDRI_API_KEY", "invalid");
        cmd.arg("sindri")
            .arg("deploy")
            .arg(circuit_path)
            .arg("--tags")
            .arg("failure");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Unauthorized"));
    }

    #[tokio::test]
    async fn test_cli_clone_circuit() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        let mock_server = wiremock::MockServer::start().await;

        let circuit_id = "mycircuit:tag";

        // Setup mock response
        let circuit_body = std::fs::read("tests/factory/circuit.tar.gz").unwrap();
        wiremock::Mock::given(method("GET"))
            .and(path(format!(
                "/api/v1/circuit/{}/download",
                urlencoding::encode(circuit_id)
            )))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(circuit_body))
            .mount(&mock_server)
            .await;

        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();
        cmd.arg("sindri")
            .arg("clone")
            .arg(circuit_id)
            .arg("--directory")
            .arg(dir_path.join("circuit").to_string_lossy().to_string())
            .arg("--base-url")
            .arg(mock_server.uri());

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Circuit downloaded to:"));

        // Check that dir_path/sindri.json exists
        let sindri_json_path = dir_path.join("circuit").join("sindri.json");
        assert!(sindri_json_path.exists());

        // Check that dir_path/circuit.circom exists
        let circuit_circom_path = dir_path.join("circuit").join("circuit.circom");
        assert!(circuit_circom_path.exists());
    }

    #[tokio::test]
    async fn test_cli_clone_bad_identifiers() {
        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();
        cmd.arg("sindri").arg("clone").arg("this/wont/work");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Invalid circuit identifier"));

        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();
        cmd.arg("sindri").arg("clone").arg("ಠ_ಠ");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Invalid circuit identifier"));
    }
}
