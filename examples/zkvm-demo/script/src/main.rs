use funty::Fundamental; // u8 to bool
use sindri::{client::SindriClient, integrations::sp1_v5::SP1ProofInfo, JobStatus, ProofInput};
use sp1_sdk::SP1Stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_emails = vec![
        "user@example.com",        // Valid
        "user.name+tag@gmail.com", // Valid
        "invalid.email@",          // Invalid
        "@nouser.com",             // Invalid
        "spaces in@email.com",     // Invalid
        "user@sub.domain.co.uk",   // Valid
    ];
    let email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    let mut handles = vec![];
    println!("Submitting {} emails", test_emails.len());
    for email in test_emails {
        let handle = tokio::spawn(async move {
            // Your API key is supplied from the environment variable SINDRI_API_KEY
            let client = SindriClient::default();

            let mut stdin = SP1Stdin::new();
            stdin.write(&email_pattern);
            stdin.write(&email);
            let proof_input = ProofInput::try_from(stdin).unwrap();

            let proof_info = client
                .prove_circuit(
                    "sindri-rust/zkvm-demo:v5", // Maps the request to the prebuilt public project
                    proof_input,
                    None, // Don't need to attach meta data
                    None, // Don't require server-side validity check
                    None, // No custom prover implementation
                )
                .await;

            if proof_info.is_err() {
                println!(
                    "Error submitting proof request for {}: {:?}",
                    email,
                    proof_info.unwrap().error
                );
                return;
            }
            let unwrapped_proof_info = proof_info.unwrap();
            if unwrapped_proof_info.status == JobStatus::Failed {
                println!(
                    "Proof generation failed for {}: {:?}",
                    email, unwrapped_proof_info.error
                );
            } else {
                let sp1_proof = unwrapped_proof_info.to_sp1_proof_with_public().unwrap();
                let sindri_verifying_key = unwrapped_proof_info.get_sp1_verifying_key().unwrap();

                if unwrapped_proof_info
                    .verify_sp1_proof_locally(&sindri_verifying_key)
                    .is_err()
                {
                    println!(
                        "Proof verification failed for {}: {:?}",
                        email, unwrapped_proof_info.error
                    );
                } else {
                    let public_values = sp1_proof.public_values.to_vec();
                    let email_valid = public_values[0].as_bool();
                    let color_code = if email_valid { "\x1b[32m" } else { "\x1b[31m" }; // Green for valid, Red for invalid
                    let reset_code = "\x1b[0m"; // Reset color
                    println!(
                        " ✓ Email '{}' is {}{}{} (ZKP verified)",
                        email,
                        color_code,
                        if email_valid { "valid" } else { "invalid" },
                        reset_code
                    );
                }
            }
        });
        handles.push(handle);
    }

    // Gather proof generation results
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
