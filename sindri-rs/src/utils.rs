use std::{error::Error, io::Read, path::Path};

use flate2::{write::GzEncoder, Compression};
use ignore::WalkBuilder;

// Global recommended maximum on circuit uploads
const MAX_PROJECT_SIZE: usize = 8 * 1024 * 1024 * 1024; // 8GB
// Designated names for special purpose files
pub(crate) const SINDRI_IGNORE_FILENAME: &str = ".sindriignore";
pub(crate) const SINDRI_MANIFEST_FILENAME: &str = "sindri.json";

/// When a user submits a path to the circuit create method, we prepare the directory
/// of the circuit project as a compressed tarfile which is sent as multipart/form data.
/// In order to fail fast, we first perform validation checks on the project directory.
/// Ignore conventions: any patterns in .gitignore, .sindriignore, are respected.
/// Hidden files are ignored.
pub async fn compress_directory(
    dir: &Path,
    override_max_project_size: Option<usize>,
) -> Result<Vec<u8>, Box<dyn Error>> {
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

    let mut contents = Vec::new();
    {
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
                let relative_path = path.strip_prefix(dir.parent().unwrap())?;
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

    Ok(contents)
}
