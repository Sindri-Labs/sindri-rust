use assert_cmd::prelude::*; 
use predicates::prelude::*; 
use std::process::Command;

#[tokio::test]
async fn test_cli_deploy() -> Result<(), Box<dyn std::error::Error>> {
    // First build the cargo-sindri binary
    let mut cmd = Command::cargo_bin("cargo-sindri")?;

    let circuit_path = "tests/factory/circuit.tar.gz";

    cmd.arg("sindri").arg("deploy").arg(circuit_path).arg("--tags").arg("success");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Circuit created successfully!"));

    Ok(())
}
