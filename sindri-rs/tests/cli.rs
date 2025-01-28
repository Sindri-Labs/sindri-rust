use assert_cmd::prelude::*; 
use predicates::prelude::*; 
use std::process::Command;

use sindri_rs::client::SindriClient;

#[tokio::test]
async fn test_cli_deploy() -> Result<(), Box<dyn std::error::Error>> {
    // First build the cargo-sindri binary
    let mut cmd = Command::cargo_bin("cargo-sindri")?;

    let circuit_path = "tests/factory/circuit.tar.gz";

    cmd.arg("sindri").arg("deploy").arg(circuit_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Circuit created successfully!"));

    // Get the circuit UUID from the output
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let uuid = stdout.lines().find(|line| line.contains("the circuit UUID")).unwrap().split("UUID: ").nth(1).unwrap();

    // Cleanup: Delete the circuit via the UUID
    let client = SindriClient::new(None, None);
    client.delete_circuit(uuid).await?;

    Ok(())
}

#[tokio::test]
async fn test_cli_deploy_unauthorized() -> Result<(), Box<dyn std::error::Error>> {
    // First build the cargo-sindri binary
    let mut cmd = Command::cargo_bin("cargo-sindri")?;

    let circuit_path = "tests/factory/circuit.tar.gz";

    cmd.env("SINDRI_API_KEY", "invalid");
    cmd.arg("sindri").arg("deploy").arg(circuit_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unauthorized"));

    Ok(())
}
