use std::collections::HashMap;

use clap::{command, Parser, Subcommand};

use sindri_rs::{
    CircuitInfo,
    client::{AuthOptions, SindriClient},
};


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
        Commands::Deploy {
            project,
            tags,
            meta,
        } => {
            // Convert metadata strings into HashMap
            let metadata = meta.map(|pairs| {
                pairs
                    .into_iter()
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((
                            parts.next()?.to_string(),
                            parts.next().unwrap_or_default().to_string(),
                        ))
                    })
                    .collect::<HashMap<String, String>>()
            });

            // Create circuit and block until complete
            match client.create_circuit_blocking(project, tags, metadata) {
                Ok(response) => {
                    // Gather circuit identifiers from response
                    let uuid = response.id();
                    let team = response.team_slug();
                    let project_name = response.project_name();
                    let first_tag = response.tags().first().cloned().unwrap_or_default();

                    println!("Circuit created successfully!");
                    println!("To generate a proof from this deployment, you can use either");
                    println!("the circuit UUID: {}", uuid);
                    println!("or identifier: {}/{}:{}", team, project_name, first_tag);
                }
                Err(e) => {
                    eprintln!("Error creating circuit\n {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
} 

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; 
    use predicates::prelude::*; 
    use std::process::Command;

    use wiremock::{
        matchers::{method, path},
        ResponseTemplate,
    };

    #[tokio::test]
    async fn test_cli_deploy_unauthorized() {
        let mock_server = wiremock::MockServer::start().await;

        // Setup mock responses
        wiremock::Mock::given(method("POST"))
            .and(path("/api/v1/circuit/create"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();

        let circuit_path = "tests/factory/circuit.tar.gz";
    
        cmd.env("SINDRI_API_KEY", "invalid");
        cmd.arg("sindri").arg("deploy").arg(circuit_path).arg("--tags").arg("failure");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Unauthorized"));

    }


}
