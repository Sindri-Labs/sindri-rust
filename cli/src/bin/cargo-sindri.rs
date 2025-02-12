use std::collections::HashMap;

use clap::{command, Parser, Subcommand};
use console::style;
use regex::Regex;

use sindri_rs::{
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

fn handle_circuit_error(message: &str) -> ! {
    eprintln!("{}", style("Circuit creation failed ❌").bold());
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

    match args.command {
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
                            handle_circuit_error(&format!(
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
                        handle_circuit_error(&response.error().unwrap_or_default())
                    }
                }
                Err(e) => handle_circuit_error(&e.to_string()),
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
        cmd.arg("sindri")
            .arg("deploy")
            .arg(circuit_path)
            .arg("--tags")
            .arg("failure");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Unauthorized"));
    }
}
