//! Utility functions for Sindri Rust SDK (mainly local file managers called by client methods)

use std::{error::Error, io::Read, path::Path};

#[cfg(feature = "rich-terminal")]
use console::style;
use flate2::{write::GzEncoder, Compression};
use ignore::WalkBuilder;
#[cfg(feature = "rich-terminal")]
use indicatif::{ProgressBar, ProgressStyle};

// Global recommended maximum on circuit uploads
const MAX_PROJECT_SIZE: usize = 8 * 1024 * 1024 * 1024; // 8GB

// Designated names for special purpose files
pub const SINDRI_IGNORE_FILENAME: &str = ".sindriignore";
pub const SINDRI_MANIFEST_FILENAME: &str = "sindri.json";
pub const CLOCK_TICKS: [&str; 12] = [
    "  🕛 ", "  🕐 ", "  🕑 ", "  🕒 ", "  🕓 ", "  🕔 ", "  🕕 ", "  🕖 ", "  🕗 ", "  🕘 ",
    "  🕙 ", "  🕚 ",
];

/// Formats bytes into human readable string with appropriate unit
#[cfg(feature = "rich-terminal")]
fn format_size(bytes: usize) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as usize, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// When a user submits a path to the circuit create method, we prepare the directory
/// of the circuit project as a compressed tarfile which is sent as multipart/form data.
///
/// Validation checks ensure the project contains a valid Sindri manifest and that the upload
/// size is within the allowed limits (8Gb by default).
///
/// If the project contains a .sindriignore file, that file is treated in the convention of .gitignore.
/// Files matchings those patterns are not included in the upload to Sindri.  
/// Hidden and .gitignored files are similarly not included.
pub async fn compress_directory(
    dir: &Path,
    override_max_project_size: Option<usize>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    #[cfg(feature = "rich-terminal")]
    println!("{}", style("Preparing circuit files...").bold());
    // Check for Sindri manifest
    let manifest_path = dir.join(SINDRI_MANIFEST_FILENAME);
    if !manifest_path.exists() {
        return Err(format!("{} not found in project root", SINDRI_MANIFEST_FILENAME).into());
    }

    // Validate JSON
    let mut manifest_file = std::fs::File::open(&manifest_path)?;
    let mut manifest_contents = String::new();
    manifest_file.read_to_string(&mut manifest_contents)?;

    serde_json::from_str::<serde_json::Value>(&manifest_contents)
        .map_err(|e| format!("Invalid JSON in {}: {}", SINDRI_MANIFEST_FILENAME, e))?;

    #[cfg(feature = "rich-terminal")]
    println!("{}", style("  ✓ Valid Sindri manifest found").cyan());

    let mut contents = Vec::new();
    {
        #[cfg(feature = "rich-terminal")]
        let pb = ProgressBar::new_spinner();
        #[cfg(feature = "rich-terminal")]
        pb.set_style(
            ProgressStyle::with_template("{spinner} {msg:.cyan}")
                .unwrap()
                .tick_strings(&crate::utils::CLOCK_TICKS),
        );
        #[cfg(feature = "rich-terminal")]
        pb.set_message("Compressing project files...");

        let buffer = std::io::Cursor::new(&mut contents);
        let enc = GzEncoder::new(buffer, Compression::default());
        let mut tar = tar::Builder::new(enc);

        // walk the directory with exclusions
        // hidden, git_ignore, git_exclude, etc are all on by default
        let walker = WalkBuilder::new(dir)
            .add_custom_ignore_filename(SINDRI_IGNORE_FILENAME)
            .build();

        for entry in walker.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                let relative_path = if dir == Path::new(".") {
                    Path::new("project").join(path.strip_prefix(dir)?)
                } else {
                    path.strip_prefix(dir.parent().unwrap())?.to_path_buf()
                };
                tar.append_file(relative_path, &mut std::fs::File::open(path)?)?;
            }
        }
    }

    // Check the size of the upload
    if contents.len() > override_max_project_size.unwrap_or(MAX_PROJECT_SIZE) {
        return Err(format!(
            "This project directory exceeds the maximum allowed size of {} and requires a special compilation process. \
            Please reach out to the Sindri team if you would like to compile the entire project \
            or double check the contents of the project for files and directories that do not \
            need to be included. Those may be added to a `{}` if you would like to \
            automatically exclude them on your next upload.", MAX_PROJECT_SIZE, SINDRI_IGNORE_FILENAME
        ).into());
    }

    #[cfg(feature = "rich-terminal")]
    println!(
        "{}",
        style(format!(
            "  ✓ Successfully prepared {} upload",
            format_size(contents.len())
        ))
        .cyan()
    );

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::SindriClient;

    use std::{
        fs::{self, File},
        io::{Cursor, Write},
        path::PathBuf,
    };

    use flate2::read::GzDecoder;
    use tar::Archive;
    use tempfile::TempDir;

    fn create_test_directory() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        // Create a valid sindri.json
        let manifest_content = r#"{"name": "test-circuit", "circuitType": "circom"}"#;
        let manifest_path = dir_path.join(SINDRI_MANIFEST_FILENAME);
        let mut file = File::create(manifest_path).unwrap();
        file.write_all(manifest_content.as_bytes()).unwrap();

        // Create some test files
        let test_file_path = dir_path.join("some_artifact.circom");
        let mut file = File::create(test_file_path).unwrap();
        file.write_all(b"test content").unwrap();

        (temp_dir, dir_path)
    }

    #[tokio::test]
    async fn test_successful_compression() {
        let (_temp_dir, dir_path) = create_test_directory();

        let result = compress_directory(&dir_path, None).await;
        assert!(result.is_ok());

        let compressed_data = result.unwrap();
        assert!(!compressed_data.is_empty());
    }

    #[tokio::test]
    async fn test_missing_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        let result = compress_directory(&dir_path, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_invalid_json_manifest() {
        let (_temp_dir, dir_path) = create_test_directory();

        // Overwrite with invalid JSON
        let manifest_path = dir_path.join(SINDRI_MANIFEST_FILENAME);
        fs::write(manifest_path, "nonjson").unwrap();

        let result = compress_directory(&dir_path, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
    }

    #[tokio::test]
    async fn test_sindriignore_respected() {
        let (_temp_dir, dir_path) = create_test_directory();

        // Create .sindriignore and a file to be ignored
        let ignore_content = "ignored.txt";
        fs::write(dir_path.join(SINDRI_IGNORE_FILENAME), ignore_content).unwrap();
        fs::write(dir_path.join("ignored.txt"), "should be ignored").unwrap();

        let circuit = compress_directory(&dir_path, None).await;
        assert!(circuit.is_ok());

        let cursor = Cursor::new(circuit.unwrap());
        let gz_decoder = GzDecoder::new(cursor);
        let mut archive = Archive::new(gz_decoder);

        let file_names: Vec<String> = archive
            .entries()
            .unwrap()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.path().ok().map(|p| p.to_string_lossy().into_owned()))
            .collect();

        assert!(!file_names.contains(&"ignored.txt".to_string()));
    }

    #[tokio::test]
    async fn test_hidden_files_ignored() {
        let (_temp_dir, dir_path) = create_test_directory();

        fs::write(dir_path.join(".hidden"), "hidden content").unwrap();

        let circuit = compress_directory(&dir_path, None).await;
        assert!(circuit.is_ok());

        let cursor = Cursor::new(circuit.unwrap());
        let gz_decoder = GzDecoder::new(cursor);
        let mut archive = Archive::new(gz_decoder);

        let file_names: Vec<String> = archive
            .entries()
            .unwrap()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.path().ok().map(|p| p.to_string_lossy().into_owned()))
            .collect();

        assert!(!file_names.contains(&".hidden".to_string()));
    }

    #[tokio::test]
    async fn test_max_project_size_exceeded() {
        let (_temp_dir, dir_path) = create_test_directory();

        // Create a file that's intentionally too large
        let test_file_path = dir_path.join("large_file.txt");
        let content: String = (0..1000).map(|_| rand::random::<u8>() as char).collect();
        fs::write(test_file_path, content).unwrap();

        // Set max size to 100 bytes
        let result = compress_directory(&dir_path, Some(100)).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("project directory exceeds"));
    }

    #[tokio::test]
    async fn test_create_circuit_invalid_file() {
        let (_temp_dir, dir_path) = create_test_directory();
        let test_file_path = dir_path.join("some_artifact.circom");

        let client = SindriClient::new(None, None);
        let result = client
            .create_circuit(test_file_path.to_string_lossy().to_string(), None, None)
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a zip file or tarball"));
    }

    #[tokio::test]
    async fn test_create_circuit_nonexistent_path() {
        let client = SindriClient::new(None, None);
        let result = client
            .create_circuit("nonexistent/path".to_string(), None, None)
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a file or directory"));
    }
}
