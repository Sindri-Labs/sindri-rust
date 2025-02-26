use sindri::{client::SindriClient, integrations::sp1::SP1ProofInfo, ProofInput};
use sp1_sdk::SP1Stdin;
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the tracing subscriber to optionally seeSindriClient logs
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Create a new Sindri client
    // Your api key is read from the SINDRI_API_KEY environment variable
    let client = SindriClient::default();

    // For more details on this public circuit, visit https://sindri.app/z/sindri/not_equal_guest/
    let project_team = "sindri";
    let project_name = "not_equal_guest";
    let circuit_tag = "latest"; // This is the default tag

    // Since we are sending two unequal inputs, we expect the boolean output to be false
    let x = 1000u32;
    let y = 2000u32;
    let mut stdin = SP1Stdin::new();
    stdin.write(&x);
    stdin.write(&y);
    let proof_input = ProofInput::try_from(stdin)?;

    let proof_info = client.prove_circuit_blocking(
        format!("{project_team}/{project_name}:{circuit_tag}").as_str(),
        proof_input,
        None, // Don't need to attach meta data
        None, // Don't require server-side validity check
        None, // No custom prover implementation
    )?;

    // Convert the proof to SP1ProofWithPublicValues and get the verifying key
    let sp1_proof = proof_info.to_sp1_proof_with_public()?;
    let sindri_verifying_key = proof_info.get_sp1_verifying_key()?;

    println!("Successfully converted Sindri proof to SP1ProofWithPublicValues");
    println!("Public values: {:?}", sp1_proof.public_values);

    // Verify the proof locally
    proof_info.verify_sp1_proof_locally(&sindri_verifying_key)?;
    println!("Proof verified successfully");

    Ok(())
}
