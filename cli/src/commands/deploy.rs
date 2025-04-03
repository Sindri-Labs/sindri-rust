use std::collections::HashMap;

use regex::Regex;
use sindri::{client::SindriClient, CircuitInfo, JobStatus};

use crate::handle_operation_error;

pub fn deploy(
    client: &SindriClient,
    project: String,
    tags: Option<Vec<String>>,
    meta: Option<Vec<String>>,
) {
    println!("{}", console::style("Deploying...").bold());

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
                    handle_operation_error(
                        "Deploy",
                        &format!("\"{pair}\" is not a valid metadata pair."),
                    );
                }
            })
            .collect::<HashMap<String, String>>()
    });
    println!(
        "{}",
        console::style(format!(
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

                println!(
                    "{}",
                    console::style("  ✓ Circuit created successfully!").cyan()
                );
                println!(
                    "\n{}",
                    console::style("To generate a proof from this deployment, you can use either:")
                        .bold()
                );
                println!("• Circuit UUID: {}", console::style(uuid).cyan());
                println!(
                    "• Identifier:  {}",
                    console::style(format!("{}/{}:{}", team, project_name, first_tag)).cyan()
                );
            } else {
                handle_operation_error("Deploy", &response.error().unwrap_or_default())
            }
        }
        Err(e) => handle_operation_error("Deploy", &e.to_string()),
    }
}
