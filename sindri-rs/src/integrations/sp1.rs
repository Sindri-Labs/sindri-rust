use crate::{
    types::{ProofInfo, ProofInfoResponse, ProofInput},
    CircuitInfoResponse, InternalProofInput,
};
use sp1_sdk_v3::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

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
        let manual_vkey = r#"
        {
            "vk": {
                "commit": {
                        "value": [1209974576, 1699531240, 1324110888, 637989123, 62921783, 507849012, 849436694, 1215033405],
                        "_marker": null
                    },
                    "pc_start": 2103080,
                    "chip_information": [
                        ["MemoryProgram", {"log_n": 19, "shift": 1}, {"width": 6, "height": 524288}],
                        ["Program", {"log_n": 19, "shift": 1}, {"width": 37, "height": 524288}],
                        ["Byte", {"log_n": 16, "shift": 1}, {"width": 11, "height": 65536}]
                    ],
                    "chip_ordering": {
                        "Byte": 2,
                        "MemoryProgram": 0,
                        "Program": 1
                }
            }
        }"#;
        let manual_proof = r#"
        {
            "proof": "lIGlUGxvbmuUktlKNDk3MzA2MTg5MTUyMzA1NDU5MjE1Mjg2MzQ0ODYxNTc1NjE3NjE4OTM4Mzc2OTg3NDA1MzYwOTI0NjI1ODM0NzgwOTMxMjk5MzHZTDY0MjQzMzYwNzk0MjQxNjc5MDI4OTAzNDc5MjYzMTcyOTAwNjAxNjUwOTk4MDU5NDA2MjAwNTk4Mzk3MTY2NjAyNDkzNjU1NTMxODHaBsAyZjJlYmFlNTI0ODE3YmZkNzQ3MzgxMTQ0MmE1NTY2Njc1M2YyOGRlYWM1YjRmY2RjMmU3YTI2ZTA2ZmM5ODM1MDYzYmE1YmViYjI3NTA0MjkzZGNhNDdkNjJlZjNjNGExNzY4MmZiNmI0MTM0M2JkN2Y5YzkyYmViOTI1ZGRkZjJkNzNiOTcyOTY5ZDZiMGI5ZTRhZjI0NzM0NTNiOTFhYmU1YTQ5YzdmZDUwYTllZjg1ZGRiZDQ4OGE5MzFjNmYwYTJhNDhiMTE0MGYyMzYyZThhNGNhYWMzMWRkZWY0MTdlMWI3M2ZjM2FmMzZmYTllMmQwYmRhZjZlOGI5MDY1Mjc4NzcwYjZjY2VlMjg5MTdhNDVmMjYzZjFiYTQ0MGNjZWZhZjMwNmI2NmEwNjhmMTIzNzAxMmRiMTJmZTgxYjBlNTIyNDgxYmRlMDExM2QyY2QwNzk5YWVmMzkyMzU1NjU4YzAyMGZlMGFjZGIxMzEzY2EzZTdmNWRmNGE2ZGYxNjZjNmI2NDc4ZjBhYjA1ZjgzZTFlMzc4MzNiMmFiY2IwOTQ1NzI2OTAyY2QyODBhZDIzZGIwZmNlZjA3ZTUzMjU1M2FmNTE1YjFiY2RkN2E3Y2E0NmQ4MWY2MzQ3MjVjNjJhZmJhYzFkMDI0YjVhMmY2OGY1MGJjZDdmYjU0ZjIzNzAwZmJjOGZiM2U1ZTQ3ODI1ZGFmODg5MmJjNzA3YTQxZjg2YWU2OGViNjhmOGFkMTAyOTU2NTMwN2Y4NTMxMDA4OGEwZDIxY2QzY2Q2MGUzZmQzNmIyZWRmZWMzNzBkMjUyZTg0ZGEyMTNmYzM3NWJjZmU1ZmEzOTAxZGU0MmRmZjkxNGViY2YxMmEzZGJhZmNjYTg0YmQxMTMyOGZhYTE3NzI3MWNmODUxMDY3N2Y3ZjNjZjhlNDQ0ZWM4ZTFjM2Q1MjM1Yjk4N2MzY2VkZGFkNjUyMTRhYjdlNTBiNGZhMmY2ZTYxNzA5N2YwMmNkNWE3YjE3MGUyZDBkNGYwZmExYTM5ZTNlOTY1MzQyNTVmMzAzMWQ1NDg3NzEzOWEyOTQ2NmQ1YzM5YjdkOTlkMDliZjEwMDU4MDBjYTllMWJkM2VhNWMzOTM0NjY1ZmJkYjQ5NzBlOGE1ZGQ4ODgwMmNiNmQzM2FlMTkyZjY2MGE1MzNjNTQ2OWZhNTZmNjJkNzE0ZTE2ZWNjMDJlNWZjODc4Y2E0YjAxNDAwMjk2MGY1ZmY5NGQ2OTY0ZTFmNzgyNzM4MzFiZmQwMTlkYTMyZGZhMmExOTE1Mjk4Y2NmMTZhYWE2N2ExNWZiNmJiNTJmNGVkMWMzM2UwZGMzYjRhYmQyMDkxMDVjNzk0MmVjMjlkNjA0Y2FiZDMyNWU1N2U1NTUxNzZmMzA1MmFhNzMzYmEzZjgyZDNiNmYzM2YzMzg5NDA2OTg5NDJkM2ZmZTI2ZjBmNTg3OGEwOTBkZGM4N2I4YzJkMTFlNWQ4MjQ1N2JkOWE5YTVkOWE5ODJjNWVkMGJmZTkwN2YzOGMzNzgxYWMwYzRiYTZiYzg0YjUyYTYzNTViMTMwM2E3NjE4Mjc2ZGZkMjM3NTk0ZGM5Y2RkYTY4N2U4MmFmOGUxMzgxMDRmMGFlMTc3NjUyZmJkZjBhZTU4ZDE0ODRkZTY1N2JhMjU3ZTJhMmY2NzgxYjliYjRmYWRmMmE1Nzg5OTcxNTBjMDRkZDk0MTI4YjNjODkwNDI2MmY3OGUwY2M2MWM5Nzg3MDE1YWI2NTFjZGY1NjQ4NjQxNmIxODYxNTQ4NDYyNGMyM2QxZDcwOTI4MzU4NjVkYTY4MmZmY2NjMmY0N2ZiNGQ4MTZjMDM1ODA2ZmExNjAzNjI5OGExNTNiYTY3MTU3MmMyZDdiNDZjOGM2OTgzYTFmNTY2YzUwZDZjYjEzNGJjYzlkODJhY2FhMWQ3MTM3MTI1NzcxMGVkMTE0ODBmODE3NTQwOTNiYTRjNWQ0NDA3OTczNzFlNmRhYjQwOTQ0MDA3YzQ2MDhmNWI1OTc0YjFiNzE4NzQyZDcyYTUwMzJhYmM2MTYxNTk5M2ZlYjI2ODlkODUzOTFhODMyZTExYjhhYjk3NjgzOWRkODA5NDM3ZmRjODhiNWIwN2JmMjlhZGRkM2ExNzBmMjFkMjRjMjVjMjIzYjExMzk2ZjViNTQ3YTlhNmNmYzVjYmEyOGQ5YjBiOGY0MzhhMDFmODI1ZWJjOTRlMTBkMDk5NjkyOTYyZjI5ZTdkM2EyMGJlMmVmNDRjN2FjZDRhMTI1ZTY5N2Q2NzI0YWQ3NjI5ZWTaBxAyZjJlYmFlNTI0ODE3YmZkNzQ3MzgxMTQ0MmE1NTY2Njc1M2YyOGRlYWM1YjRmY2RjMmU3YTI2ZTA2ZmM5ODM1MDYzYmE1YmViYjI3NTA0MjkzZGNhNDdkNjJlZjNjNGExNzY4MmZiNmI0MTM0M2JkN2Y5YzkyYmViOTI1ZGRkZjJkNzNiOTcyOTY5ZDZiMGI5ZTRhZjI0NzM0NTNiOTFhYmU1YTQ5YzdmZDUwYTllZjg1ZGRiZDQ4OGE5MzFjNmYwYTJhNDhiMTE0MGYyMzYyZThhNGNhYWMzMWRkZWY0MTdlMWI3M2ZjM2FmMzZmYTllMmQwYmRhZjZlOGI5MDY1Mjc4NzcwYjZjY2VlMjg5MTdhNDVmMjYzZjFiYTQ0MGNjZWZhZjMwNmI2NmEwNjhmMTIzNzAxMmRiMTJmZTgxYjBlNTIyNDgxYmRlMDExM2QyY2QwNzk5YWVmMzkyMzU1NjU4YzAyMGZlMGFjZGIxMzEzY2EzZTdmNWRmNGE2ZGYyNmYwZjU4NzhhMDkwZGRjODdiOGMyZDExZTVkODI0NTdiZDlhOWE1ZDlhOTgyYzVlZDBiZmU5MDdmMzhjMzc4MWFjMGM0YmE2YmM4NGI1MmE2MzU1YjEzMDNhNzYxODI3NmRmZDIzNzU5NGRjOWNkZGE2ODdlODJhZjhlMTM4MTE2NmM2YjY0NzhmMGFiMDVmODNlMWUzNzgzM2IyYWJjYjA5NDU3MjY5MDJjZDI4MGFkMjNkYjBmY2VmMDdlNTMyNTUzYWY1MTViMWJjZGQ3YTdjYTQ2ZDgxZjYzNDcyNWM2MmFmYmFjMWQwMjRiNWEyZjY4ZjUwYmNkN2ZiNTRmMjM3MDBmYmM4ZmIzZTVlNDc4MjVkYWY4ODkyYmM3MDdhNDFmODZhZTY4ZWI2OGY4YWQxMDI5NTY1MzA3Zjg1MzEwMDg4YTBkMjFjZDNjZDYwZTNmZDM2YjJlZGZlYzM3MGQyNTJlODRkYTIxM2ZjMzc1YmNmZTVmYTM5MDFkZTQyZGZmOTE0ZWJjZjEyYTNkYmFmY2NhODRiZDExMzI4ZmFhMTc3MjcxY2Y4NTEwNjc3ZjdmM2NmOGU0NDRlYzhlMWMzZDUyMzViOTg3YzNjZWRkYWQ2NTIxNGFiN2U1MGI0ZmEyZjZlNjE3MDk3ZjAyY2Q1YTdiMTcwZTJkMGQ0ZjBjMDRkZDk0MTI4YjNjODkwNDI2MmY3OGUwY2M2MWM5Nzg3MDE1YWI2NTFjZGY1NjQ4NjQxNmIxODYxNTQ4NDYyNGMyM2QxZDcwOTI4MzU4NjVkYTY4MmZmY2NjMmY0N2ZiNGQ4MTZjMDM1ODA2ZmExNjAzNjI5OGExNTNiYTY3MDAwMDAwMDcxMWQyZmZkOWM5YzM3M2MzMTYyN2E3YjdlZmFjNzRiMWZhMjcxN2UxMDlmMWI4Mjg3NmEwNWU1NzEyNmI5ZTVmMGZhMWEzOWUzZTk2NTM0MjU1ZjMwMzFkNTQ4NzcxMzlhMjk0NjZkNWMzOWI3ZDk5ZDA5YmYxMDA1ODAwY2E5ZTFiZDNlYTVjMzkzNDY2NWZiZGI0OTcwZThhNWRkODg4MDJjYjZkMzNhZTE5MmY2NjBhNTMzYzU0NjlmYTU2ZjYyZDcxNGUxNmVjYzAyZTVmYzg3OGNhNGIwMTQwMDI5NjBmNWZmOTRkNjk2NGUxZjc4MjczODMxYmZkMDE5ZGEzMmRmYTJhMTkxNTI5OGNjZjE2YWFhNjdhMTVmYjZiYjUyZjRlZDFjMzNlMGRjM2I0YWJkMjA5MTA1Yzc5NDJlYzI5ZDYwNGNhYmQzMjVlNTdlNTU1MTc2ZjMwNTJhYTczM2JhM2Y4MmQzYjZmMzNmMzM4OTQwNjk4OTQyZDNmZmUwMzJhYmM2MTYxNTk5M2ZlYjI2ODlkODUzOTFhODMyZTExYjhhYjk3NjgzOWRkODA5NDM3ZmRjODhiNWIwN2JmMTU3MmMyZDdiNDZjOGM2OTgzYTFmNTY2YzUwZDZjYjEzNGJjYzlkODJhY2FhMWQ3MTM3MTI1NzcxMGVkMTE0ODBmODE3NTQwOTNiYTRjNWQ0NDA3OTczNzFlNmRhYjQwOTQ0MDA3YzQ2MDhmNWI1OTc0YjFiNzE4NzQyZDcyYTUwNGYwYWUxNzc2NTJmYmRmMGFlNThkMTQ4NGRlNjU3YmEyNTdlMmEyZjY3ODFiOWJiNGZhZGYyYTU3ODk5NzE1MDAwMDAwMDEyOWFkZGQzYTE3MGYyMWQyNGMyNWMyMjNiMTEzOTZmNWI1NDdhOWE2Y2ZjNWNiYTI4ZDliMGI4ZjQzOGEwMWY4MjVlYmM5NGUxMGQwOTk2OTI5NjJmMjllN2QzYTIwYmUyZWY0NGM3YWNkNGExMjVlNjk3ZDY3MjRhZDc2MjllZNwAIFTMvczKzOPMrcy4PUzM6cztzJHM2cyaMczaMMyGzOLMsRfMq8zzaFFkzOnM8syNeGcLBZOSlMzoAwAAlMzQBwAAAJCRkZEApnYzLjAuMA=="
        }"#;

        // Parse the response and assert success
        let proof_info = ProofInfoResponse {
            circuit_type: CircuitType::Sp1,
            verification_key: serde_json::from_str(manual_vkey).expect("Failed to parse hardcoded verifying key"),
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
