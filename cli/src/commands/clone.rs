use std::{fs, io::Cursor, path::Path};

use flate2::read::GzDecoder;
use regex::Regex;
use tar::Archive;

use sindri::client::SindriClient;

use crate::handle_operation_error;

pub fn clone(client: &SindriClient, circuit: String, directory: Option<String>) {
    println!("{}", console::style("Cloning...").bold());

    let circuit_regex =
        Regex::new(r"^(?:([-a-zA-Z0-9_]+)\/)?([-a-zA-Z0-9_]+)(?::([-a-zA-Z0-9_.]+))?$").unwrap();
    let circuit_name = if let Some(captures) = circuit_regex.captures(&circuit) {
        captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| handle_operation_error("Clone", "Invalid circuit identifier"))
    } else {
        handle_operation_error("Clone", "Invalid circuit identifier")
    };
    let output_directory = directory.unwrap_or(circuit_name.clone());
    println!(
        "{}",
        console::style(format!("  ✓ Valid circuit identifier: {}", circuit)).cyan()
    );

    let download_path = {
        let p = Path::new(&output_directory);
        if p.is_dir() {
            handle_operation_error("Clone", "Output directory already exists");
        }
        match fs::create_dir_all(p) {
            Ok(_) => p.join("circuit.tar.gz"),
            Err(e) => handle_operation_error("Clone", &e.to_string()),
        }
    };

    match client.clone_circuit_blocking(&circuit, download_path.to_string_lossy().to_string()) {
        Ok(_) => println!(
            "{}",
            console::style("  ✓ Successfully downloaded circuit").cyan()
        ),
        Err(e) => {
            if e.to_string().contains("404") {
                handle_operation_error(
                    "Clone",
                    "Circuit does not exist or you lack permission to access it.",
                );
            } else {
                handle_operation_error("Clone", &e.to_string());
            }
        }
    }

    println!("{}", console::style("  ✓ Unpacking circuit...").cyan());
    // Unpack the tarball
    let downloaded = fs::read(&download_path).unwrap();
    let cursor = Cursor::new(downloaded);
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz_decoder);

    // Manually unpack the tarball, stripping the top-level directory
    (|| -> Result<(), Box<dyn std::error::Error>> {
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            if let Some(stripped) = path.iter().skip(1).collect::<std::path::PathBuf>().to_str() {
                let output_path = Path::new(&output_directory).join(stripped);
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                entry.unpack(&output_path)?;
            }
        }
        Ok(())
    })()
    .unwrap_or_else(|e| {
        handle_operation_error("Clone", &format!("Issue unpacking circuit: {}", e))
    });

    // Remove the download tarball
    std::fs::remove_file(&download_path).unwrap();

    println!(
        "{}",
        console::style("  ✓ Circuit cloned successfully!").cyan()
    );
    println!(
        "\n{}",
        console::style(format!("Circuit downloaded to: {}", output_directory)).bold()
    );
}
