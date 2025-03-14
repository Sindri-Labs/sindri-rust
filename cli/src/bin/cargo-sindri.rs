use std::collections::HashMap;

use clap::{command, Parser, Subcommand};
use console::style;
use regex::Regex;

use sindri::{
    client::{AuthOptions, SindriClient},
    CircuitInfo, JobStatus,
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
    /// Clone a circuit
    Clone {
        /// Circuit ID to clone, could be a UUID or a team/project:tag identifier
        #[arg(required = true)]
        circuit: String,

        /// Directory where the circuit should be downloaded
        #[arg(long, default_value = ".")]
        directory: String,
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

fn handle_operation_error(command: &str, message: &str) -> ! {
    eprintln!("{}", style(format!("{} failed ❌", command)).bold());
    eprintln!("{}", style(message).red());
    std::process::exit(1);
}

fn main() {
    let Cargo::Sindri(args) = Cargo::parse();

    // Initialize client with provided auth options
    let auth = AuthOptions {
        api_key: args.api_key,
        base_url: args.base_url,
    };
    let client = SindriClient::new(Some(auth), None);
    println!("{}", client.base_url());
    match args.command {
        Commands::Clone { circuit, directory } => {
            match client.clone_circuit_blocking(&circuit, directory, None) {
                Ok(_) => println!("Successfully cloned circuit {}", circuit),
                Err(e) => handle_operation_error("clone", &e.to_string()),
            }
        },
        Commands::Deploy {
            project,
            tags,
            meta,
        } => {
            println!("{}", style("Deploying...").bold());

            // Convert metadata strings into HashMap
            let meta_rules = Regex::new(r"^[a-zA-Z0-9]+=[a-zA-Z0-9]+$").unwrap();
            let metadata = meta.map(|pairs| {
                pairs
                    .into_iter()
                    .filter_map(|pair| {
                        if meta_rules.is_match(&pair) {
                            let mut parts = pair.splitn(2, '=');
                            Some((
                                parts.next()?.to_string(),
                                parts.next().unwrap_or_default().to_string(),
                            ))
                        } else {
                            handle_operation_error("deploy", &format!(
                                "\"{pair}\" is not a valid metadata pair."
                            ));
                        }
                    })
                    .collect::<HashMap<String, String>>()
            });
            println!(
                "{}",
                style(format!(
                    "  ✓ Valid metadata pairs specified: {}",
                    metadata.as_ref().map_or(0, |t| t.len())
                ))
                .cyan()
            );

            match client.create_circuit_blocking(project, tags, metadata) {
                Ok(response) => {
                    // Gather circuit identifiers from response
                    let status = *response.status();
                    if status == JobStatus::Ready {
                        let uuid = response.id();
                        let team = response.team_slug();
                        let project_name = response.project_name();
                        let first_tag = response.tags().first().cloned().unwrap_or_default();

                        println!("{}", style("  ✓ Circuit created successfully!").cyan());
                        println!(
                            "\n{}",
                            style("To generate a proof from this deployment, you can use either:")
                                .bold()
                        );
                        println!("• Circuit UUID: {}", style(uuid).cyan());
                        println!(
                            "• Identifier:  {}",
                            style(format!("{}/{}:{}", team, project_name, first_tag)).cyan()
                        );
                    } else {
                        handle_operation_error("deploy", &response.error().unwrap_or_default())
                    }
                }
                Err(e) => handle_operation_error("deploy", &e.to_string()),
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

    use sindri::{BoojumCircuitInfoResponse, CircuitInfoResponse, JobStatus};

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

        let mock_server = wiremock::MockServer::start().await;

        let circuit_id = "123e4567-e89b-12d3-a456-426614174000";

        // Setup mock responses
        wiremock::Mock::given(method("GET"))
        .and(path(format!("/api/v1/circuit/{}/detail", circuit_id)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(CircuitInfoResponse::Boojum(Box::new(
                BoojumCircuitInfoResponse {
                    status: JobStatus::Ready,
                    file_size: Some(100),
                    ..Default::default()
                },
            ))),
        )
        .mount(&mock_server)
        .await;
        
        wiremock::Mock::given(method("GET"))
            .and(path(format!("/api/v1/circuit/{}/download", circuit_id)))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let mut cmd = Command::cargo_bin("cargo-sindri").unwrap();
        cmd.arg("sindri")
            .arg("clone")
            .arg(circuit_id)
            .arg("--directory")
            .arg("output")
            .arg("--base-url")
            .arg(mock_server.uri());

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Successfully cloned circuit"));
    }
}
