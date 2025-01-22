use sp1_sdk_v3::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};
use crate::{CircuitInfoResponse, InternalProofInput, types::{ProofInfo, ProofInfoResponse, ProofInput}};

/// Convert SP1Stdin to ProofInput type accepted by SindriClient proof generation methods
impl TryFrom<SP1Stdin> for ProofInput {
    type Error = serde_json::Error;

    fn try_from(stdin: SP1Stdin) -> Result<Self, Self::Error> {
        let stdin_str = serde_json::to_string(&stdin)?;
        Ok(ProofInput(InternalProofInput::String(stdin_str)))
    }
}

/// Trait for SP1ProgramInfo
/// This trait is used to extract the SP1 verifying key from the CircuitInfoResponse
pub trait SP1ProgramInfo {
    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>>;
}

impl SP1ProgramInfo for CircuitInfoResponse {
    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>> {
        match self {
            CircuitInfoResponse::Sp1(info) => {
                let verifying_key = info.verification_key.clone()
                    .ok_or("Verifying key is not populated, possibly the program has not completed compilation")?;
                let verifying_key_sp1: SP1VerifyingKey = serde_json::from_value(verifying_key)?;
                Ok(verifying_key_sp1)
            },
            _ => Err("Circuit type is not SP1".into())
        }
    }
}

/// Trait for SP1ProofInfo
/// This trait is used to extract the SP1 proof and verifying key from the ProofInfoResponse
pub trait SP1ProofInfo {
    fn to_sp1_proof_with_public(&self) -> Result<SP1ProofWithPublicValues, Box<dyn std::error::Error>>;
    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>>;
    fn verify_sp1_proof_locally(&self, verifying_key: &SP1VerifyingKey) -> Result<(), Box<dyn std::error::Error>>;
}

impl SP1ProofInfo for ProofInfoResponse {
    fn to_sp1_proof_with_public(&self) -> Result<SP1ProofWithPublicValues, Box<dyn std::error::Error>> {
        let proof_bytes = self.get_proof_as_bytes()?;
        let proof: SP1ProofWithPublicValues = rmp_serde::from_slice(&proof_bytes)?;
        Ok(proof)
    }

    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>> {
        let verifying_key = self.verification_key.clone().flatten().ok_or("Verifying key is not populated")?;
        let verifying_key_sp1: SP1VerifyingKey = serde_json::from_value(verifying_key)?;
        Ok(verifying_key_sp1)
    }

    fn verify_sp1_proof_locally(&self, verifying_key: &SP1VerifyingKey) -> Result<(), Box<dyn std::error::Error>> {
        let local_sp1_client = ProverClient::new();
        let sp1_proof = self.to_sp1_proof_with_public()?;
        local_sp1_client.verify(&sp1_proof, verifying_key).map_err(|e| e.into())
    }
}
