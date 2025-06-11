use crate::{
    types::{ProofInfo, ProofInfoResponse, ProofInput},
    CircuitInfoResponse, InternalProofInput,
};
use sp1_sdk_v5::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

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
        let local_sp1_client = ProverClient::from_env();
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
        let manual_vkey = r#"{"vk": {"commit": {"value": [230344292, 1578938504, 1023581524, 674873028, 1876933151, 1372477700, 814573411, 352501463], "_marker": null}, "pc_start": 2104288, "initial_global_cumulative_sum": {"x": [1409220894, 16379980, 324315092, 1507275353, 1581811953, 152762095, 1178218523], "y": [1749986198, 1514596200, 1556413804, 539190630, 1158574419, 1734760409, 518985250]}, "chip_information": [["Program", {"log_n": 19, "shift": 1}, {"width": 14, "height": 524288}], ["Byte", {"log_n": 16, "shift": 1}, {"width": 11, "height": 65536}]], "chip_ordering": {"Program": 0, "Byte": 1}}}"#;
        let manual_proof = r#"{"proof": "lIGnR3JvdGgxNpSS2Us0MTgxOTUxMzQ1NzIyOTg0NjExNDQ3Mjg3NTgwNDkyMjgzNDUxMjI1NjIwMDA1NTIxMTc3MzY3Nzg1Mzk5MTEwODc3NTQ0MjA3NDDZTDY4MzU0MzM0NzMwNzI1ODI1Mzc3MzU3NzkwMDUyNTIzNzgxNzg0MDE5MjA4ODYwMDEzOTEwODM1MDYyMjIxMDAwNDExNzcxNDQ3MjDaAgAwOGVlMGU3MzdmMjAzMWQ2NzU0ODBmNWFkYjEzMTllMjIxZTM4YzdiYjFkNTAwNjJhODQzOWUzNGM0MGNiNzVkMmEzMGVkYzY5NmNiYjEyNmExNjRmMjAxNzRlN2FhMjExODA2Mzg2MjhkYjZiNWQ4YzJlNzZkZDI3ZTkwZDZmYjAwZGI2OTI5NDY2MzE1NDU2MDM5MTYwODc5NWY0MWRhMmJmZmNiZjEzZDZmZjFjNmRlN2QxYjUxMjExOGI3Y2QwM2I0ZTA1OGI2ODQ5NjUyNzQ3NzI4ODAwN2Y2OTljOWY3NjBiZWRlMGNjNGNjZDI0YWI4ZDkwYmJlMzlmNTU0MTc0MzJiYThlMWJlN2Y0NTU1NzU3ZGRiMjkzYjJiNTgzODRiMWYwZjRkMDg2Yzc2MzNmNWVmZTY0NzE5NzRkYzJlODE4MDdjZGFjODk5NTBjNTY1ZTdlODk5OWJmNGExNWIxNWI5Y2I1NWM3NzlmODViNjI5OTM3ZGY3Njk5ZDgxMTY3YWY0OTY3NzRkNjhjMDUyZTExMTRiYzk0MDIwMDgzZWYyNjEwNDg4NWRjNzhjMDEzMmMyODMyM2I0NjQ2MWU0MTM1Mjc3YmE2OTg1YzgxMTlkZDdhYTQxYjUxOWY5NzI2NWJiZmMyODI4M2RhZmJhNjczOGQwMzFjYjUwZdoCiDA4ZWUwZTczN2YyMDMxZDY3NTQ4MGY1YWRiMTMxOWUyMjFlMzhjN2JiMWQ1MDA2MmE4NDM5ZTM0YzQwY2I3NWQyYTMwZWRjNjk2Y2JiMTI2YTE2NGYyMDE3NGU3YWEyMTE4MDYzODYyOGRiNmI1ZDhjMmU3NmRkMjdlOTBkNmZiMDBkYjY5Mjk0NjYzMTU0NTYwMzkxNjA4Nzk1ZjQxZGEyYmZmY2JmMTNkNmZmMWM2ZGU3ZDFiNTEyMTE4YjdjZDAzYjRlMDU4YjY4NDk2NTI3NDc3Mjg4MDA3ZjY5OWM5Zjc2MGJlZGUwY2M0Y2NkMjRhYjhkOTBiYmUzOWY1NTQxNzQzMmJhOGUxYmU3ZjQ1NTU3NTdkZGIyOTNiMmI1ODM4NGIxZjBmNGQwODZjNzYzM2Y1ZWZlNjQ3MTk3NGRjMmU4MTgwN2NkYWM4OTk1MGM1NjVlN2U4OTk5YmY0YTE1YjE1YjljYjU1Yzc3OWY4NWI2Mjk5MzdkZjc2OTlkODExNjdhZjQ5Njc3NGQ2OGMwNTJlMTExNGJjOTQwMjAwODNlZjI2MTA0ODg1ZGM3OGMwMTMyYzI4MzIzYjQ2NDYxZTQxMzUyNzdiYTY5ODVjODExOWRkN2FhNDFiNTE5Zjk3MjY1YmJmYzI4MjgzZGFmYmE2NzM4ZDAzMWNiNTBlMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMNwAIMykWUxZzLvMwULM88y4HD7My39QzKfMw0vMycyvfExES11IzLfMlUJ+KFkTkZHcAGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABptAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAKszCpnY1LjAuMMA="}"#;

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
