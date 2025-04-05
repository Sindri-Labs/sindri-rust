use serde_json::json;
use sindri::{client::SindriClient, CircuitInfo, JobStatus};

fn main() {
    // Instruct the client to use this entire package for upload
    let dir_path = ".";

    // Your API key is supplied from the environment variable SINDRI_API_KEY
    let client = SindriClient::default();

    println!("Creating circuit...");
    let deployment = match client.create_circuit_blocking(
        dir_path.to_string(),
        None,
        None, 
    ) {
        Ok(circuit_info) => {
            circuit_info
        }
        Err(e) => {
            println!("Error deploying project: {:?}", e);
            return;
        }
    };

    if deployment.status() == &JobStatus::Failed {
        println!("Project build failed: {:?}", deployment.error());
        return;
    }

    let uuid = deployment.id();
    let project_name = deployment.project_name();

    println!("Project deployed successfully");
    println!("To request proofs from this project, you can use either:");
    println!("1. The UUID: {}", uuid);
    println!("2. The project name: {}", project_name);

    println!("\nRequesting a proof...");
    let proof_result = match client.prove_circuit_blocking(
        uuid, // The UUID assigned to the circuit during creation
        json!({"x": 1, "y": 2}), // The proving input (x != y)
        None, 
        None, 
        None
    ) {
        Ok(proof) => {
            proof
        }
        Err(e) => {
            println!("Error requesting proof: {:?}", e);
            return;
        }
    };

    if proof_result.status == JobStatus::Failed {
        println!("Proof generation failed: {:?}", proof_result.error);
        return;
    }

    println!("Proof generated successfully");
    println!("Proof: {}", proof_result.proof.unwrap().unwrap());
    println!("Public: {}", proof_result.public.unwrap().unwrap());
}
