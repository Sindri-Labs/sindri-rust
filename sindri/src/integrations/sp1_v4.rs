use crate::{
    types::{ProofInfo, ProofInfoResponse, ProofInput},
    CircuitInfoResponse, InternalProofInput,
};
use sp1_sdk_v4::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

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
            }
            _ => Err("Circuit type is not SP1".into()),
        }
    }
}

/// Trait for SP1ProofInfo
/// This trait is used to extract the SP1 proof and verifying key from the ProofInfoResponse
pub trait SP1ProofInfo {
    fn to_sp1_proof_with_public(
        &self,
    ) -> Result<SP1ProofWithPublicValues, Box<dyn std::error::Error>>;
    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>>;
    fn verify_sp1_proof_locally(
        &self,
        verifying_key: &SP1VerifyingKey,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl SP1ProofInfo for ProofInfoResponse {
    fn to_sp1_proof_with_public(
        &self,
    ) -> Result<SP1ProofWithPublicValues, Box<dyn std::error::Error>> {
        let proof_bytes = self.get_proof_as_bytes()?;
        let proof: SP1ProofWithPublicValues = rmp_serde::from_slice(&proof_bytes)?;
        Ok(proof)
    }

    fn get_sp1_verifying_key(&self) -> Result<SP1VerifyingKey, Box<dyn std::error::Error>> {
        let verifying_key = self
            .verification_key
            .clone()
            .flatten()
            .ok_or("Verifying key is not populated")?;
        let verifying_key_sp1: SP1VerifyingKey = serde_json::from_value(verifying_key)?;
        Ok(verifying_key_sp1)
    }

    fn verify_sp1_proof_locally(
        &self,
        verifying_key: &SP1VerifyingKey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let local_sp1_client = ProverClient::new();
        let sp1_proof = self.to_sp1_proof_with_public()?;
        local_sp1_client
            .verify(&sp1_proof, verifying_key)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CircuitType;

    #[test]
    fn test_sp1_stdin_conversion() {
        // Create sample SP1Stdin
        let x = 1u32;
        let mut stdin = SP1Stdin::new();
        stdin.write(&x);

        // Convert to ProofInput
        let proof_input = ProofInput::try_from(stdin).unwrap();

        // Verify the conversion
        if let ProofInput(InternalProofInput::String(stdin_str)) = proof_input {
            let roundtrip: SP1Stdin = serde_json::from_str(&stdin_str).unwrap();
            let value = u32::from_le_bytes(roundtrip.buffer[0][..4].try_into().unwrap());
            assert_eq!(value, x);
        } else {
            panic!("ProofInput should contain InternalProofInput::String variant");
        }
    }

    #[test]
    fn test_sp1_proof_info() {
        let manual_vkey = r#"{"vk": {"commit": {"value": [1655052977, 1821085182, 385492598, 1724952401, 447395258, 1679068494, 1142928577, 1854966442], "_marker": null}, "pc_start": 2102232, "initial_global_cumulative_sum": {"x": [151757511, 318756418, 1431527115, 783397551, 1836138840, 1208372966, 485183523], "y": [644109448, 439496439, 713063277, 1268218131, 906615174, 195214167, 1387799099]}, "chip_information": [["Program", {"log_n": 19, "shift": 1}, {"width": 14, "height": 524288}], ["Byte", {"log_n": 16, "shift": 1}, {"width": 11, "height": 65536}]], "chip_ordering": {"Program": 0, "Byte": 1}}}"#;
        let manual_proof = r#"{"proof": "k4GnR3JvdGgxNpSS2UsxNzE3NTg3NjY0NTg2ODQyODY4MzM2MzU2NzYyMjYzOTkzMjczODc1ODcyNDU2MDM4NDM0OTgwMjg1MTYwNjkyNjA0MjMxMzc4ODHZTDY4MzU0MzM0NzMwNzI1ODI1Mzc3MzU3NzkwMDUyNTIzNzgxNzg0MDE5MjA4ODYwMDEzOTEwODM1MDYyMjIxMDAwNDExNzcxNDQ3MjDaAgAxYzVmNDQzY2M5MTQyNThhOWRiYjc0NmExMGU1YmY2YmM0NjM3OGZlMjZkMjZkYTM3ZWQwZWQ1NWQ3NTYwMTk1MWUxYzU1ZThkMDNjMzVmMWQ5ZjRiYTEzMmE1YmY1ZDc0OWU5MTU5ZWU1YjE4Zjg2OWQxNTYzMWYyNzFkMGU3ZTIwYzRiZDI3MmViZjk2YmM4NjA0MzRmZDUxYTg3ZGMwNjJhY2EzMGUxOGUzYmJlYjE3YWI5OGRiYmMzOGRkNGQyYTRhNGQzNGUzM2NmNzk4NGZmYjI2Y2ViZDNjYjNjY2IzNzYzODBmYzNmNDVhOTQ5NzY0NTk1YWRlN2EyZThhMWI3ZWQ3MmZhOWZlOGE4YTFjMWY3OTY2ZDI2ZjI2OTZhZDI5ZjhkNjIxOWQ4M2Y4OGMzYzU3NDU3NzU4ZTVlMzJiNDBjYWZmZTYzODYwMDU4NDBmYmE0ZjcwMjQ3NDhjNTUwMGFkMjc2NjI4NTU2MGM3NWMyNGU3MDg0ZjUwMzkxYjAxYzdlYjQzMmE1MjhmMDVhNDY3YjMyNjI0NTY5YmE0NjIwYzkxMjk1NTNlOTlhMjRmZWI0NTNmNTU3OGY5MDVhMTQ0MDdjN2JkODMxMDk2OGQzYmVhZGU1ODM4MDljNDE1ZDU0MGI0N2NlZjhhMjUwZmJhMGRkODM4NjA2M9oCiDFjNWY0NDNjYzkxNDI1OGE5ZGJiNzQ2YTEwZTViZjZiYzQ2Mzc4ZmUyNmQyNmRhMzdlZDBlZDU1ZDc1NjAxOTUxZTFjNTVlOGQwM2MzNWYxZDlmNGJhMTMyYTViZjVkNzQ5ZTkxNTllZTViMThmODY5ZDE1NjMxZjI3MWQwZTdlMjBjNGJkMjcyZWJmOTZiYzg2MDQzNGZkNTFhODdkYzA2MmFjYTMwZTE4ZTNiYmViMTdhYjk4ZGJiYzM4ZGQ0ZDJhNGE0ZDM0ZTMzY2Y3OTg0ZmZiMjZjZWJkM2NiM2NjYjM3NjM4MGZjM2Y0NWE5NDk3NjQ1OTVhZGU3YTJlOGExYjdlZDcyZmE5ZmU4YThhMWMxZjc5NjZkMjZmMjY5NmFkMjlmOGQ2MjE5ZDgzZjg4YzNjNTc0NTc3NThlNWUzMmI0MGNhZmZlNjM4NjAwNTg0MGZiYTRmNzAyNDc0OGM1NTAwYWQyNzY2Mjg1NTYwYzc1YzI0ZTcwODRmNTAzOTFiMDFjN2ViNDMyYTUyOGYwNWE0NjdiMzI2MjQ1NjliYTQ2MjBjOTEyOTU1M2U5OWEyNGZlYjQ1M2Y1NTc4ZjkwNWExNDQwN2M3YmQ4MzEwOTY4ZDNiZWFkZTU4MzgwOWM0MTVkNTQwYjQ3Y2VmOGEyNTBmYmEwZGQ4Mzg2MDYzMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMNwAIBHMtsygzJ1jzNJVzK1CXszjzKfM9iEdXszGP8y9zOnMgFtAVRwxNidbb07MtJGR3ABgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAabQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACrMwqt2NC4wLjAtcmMuMw=="}"#;

        // Parse the response and assert success
        let proof_info = ProofInfoResponse {
            circuit_type: CircuitType::Sp1,
            verification_key: serde_json::from_str(manual_vkey)
                .expect("Failed to parse hardcoded verifying key"),
            proof: serde_json::from_str(manual_proof).expect("Failed to parse mock response JSON"),
            ..Default::default()
        };

        // Extract proof and verify it can be converted to SP1 format
        let _ = proof_info
            .to_sp1_proof_with_public()
            .expect("Failed to convert proof to SP1 format");

        // Extract and validate verifying key
        let sp1_verifying_key = proof_info
            .get_sp1_verifying_key()
            .expect("Failed to get SP1 verifying key");

        // Verify the proof locally
        proof_info
            .verify_sp1_proof_locally(&sp1_verifying_key)
            .expect("Failed to verify SP1 proof locally");
    }
}
