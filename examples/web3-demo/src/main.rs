use alloy::{
    network::{ReceiptResponse, TransactionBuilder},
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::Result;
use serde_json::json;
use sindri::client::SindriClient;

mod sudoku_io;
use sudoku_io::get_sudoku_solution;
mod types;
use types::{convert_public, CircomProof, CircomProofLite};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Verifier,
    "contract/verifier.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Play a local game of sudoku & then reset the terminal
    let solution = match get_sudoku_solution() {
        Some(solution) => solution,
        None => return Err(eyre::eyre!("No solution provided")),
    };
    println!();

    // Your API key is supplied from the environment variable SINDRI_API_KEY
    let client = SindriClient::default();

    println!("Requesting a ZKP for the solution: {:?}", solution);
    let proof = match client
        .prove_circuit(
            "sindri-rust/web3-demo", // Maps the request to the prebuilt public project
            json!({
                "puzzle": sudoku_io::PUZZLE.to_vec(),
                "solution": solution
            }), // JSON proving input
            None,                              // Optional metadata
            None,                              // Server-side proof verification
            None,
        ) // Custom prover implementations
        .await
    {
        Ok(proof) => proof,
        Err(e) => {
            println!("Error requesting or waiting for proof: {:?}", e);
            return Err(eyre::eyre!("{}", e));
        }
    };

    let circom_proof: CircomProofLite = match proof.proof {
        Some(Some(proof)) => serde_json::from_value::<CircomProof>(proof)
            .unwrap()
            .to_lite(),
        _ => {
            println!("Proof generation failed!");
            if let Some(Some(error)) = proof.error {
                println!("Error details: {}", error);
            }
            return Err(eyre::eyre!("Failed to generate proof"));
        }
    };
    let circom_public = match proof.public {
        Some(Some(ref public)) => convert_public(*public.clone()).unwrap(),
        _ => return Err(eyre::eyre!("No public input provided")),
    };
    match circom_public[0].to::<bool>() {
        true => println!("Solution is valid!\n"),
        false => println!("Solution is invalid.\n"),
    };

    println!("Producing on-chain verification...");
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH by installing foundry.
    let provider = ProviderBuilder::new().on_anvil_with_wallet();

    // Deploy the verifier contract.
    let contract = Verifier::deploy(&provider).await?;
    println!("Deployed contract at address: {}", contract.address());

    println!("Verifying proof...");
    let builder = contract.verifyProof(
        circom_proof.pi_a,
        circom_proof.pi_b,
        circom_proof.pi_c,
        circom_public,
    );
    // See the `verifyProof` function in the `Verifier` contract:
    // @return r  bool true if proof is valid
    let verification_result = builder.call().await?.r; 
    if verification_result {
        println!("Proof verified successfully on chain!");
    } else {
        println!("Proof failed smart contract verification!");
    }

    Ok(())
}
