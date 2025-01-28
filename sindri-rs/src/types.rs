//! Common types re-exported from the openapi (internal) package.
use base64::engine::{general_purpose, Engine};
pub use openapi::models::{
    BoojumCircuitInfoResponse, CircomCircuitInfoResponse, CircuitInfoResponse, CircuitType,
    GnarkCircuitInfoResponse, Halo2CircuitInfoResponse, JobStatus, JoltCircuitInfoResponse,
    NoirCircuitInfoResponse, Plonky2CircuitInfoResponse, ProofInfoResponse,
    ProofInput as InternalProofInput, Sp1CircuitInfoResponse,
};
use std::collections::HashMap;

/// Helper trait to extract common fields from CircuitInfoResponse
pub trait CircuitInfo {
    fn compute_time_sec(&self) -> Option<f64>;
    fn date_created(&self) -> &str;
    fn error(&self) -> Option<String>;
    fn file_size(&self) -> Option<i32>;
    fn finished_processing(&self) -> bool;
    fn id(&self) -> &str;
    fn meta(&self) -> &HashMap<String, String>;
    fn num_proofs(&self) -> Option<i32>;
    fn project_name(&self) -> &str;
    fn proving_scheme(&self) -> &str;
    fn queue_time_sec(&self) -> Option<f64>;
    fn status(&self) -> &JobStatus;
    fn tags(&self) -> &Vec<String>;
    fn team(&self) -> &str;
}

macro_rules! impl_circuit_info {
    ($($variant:ident),*) => {
        impl CircuitInfo for CircuitInfoResponse {

            fn compute_time_sec(&self) -> Option<f64> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.compute_time_sec,
                    )*
                }
            }

            fn date_created(&self) -> &str {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.date_created,
                    )*
                }
            }

            fn error(&self) -> Option<String> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.error.clone(),
                    )*
                }
            }

            fn file_size(&self) -> Option<i32> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.file_size,
                    )*
                }
            }

            fn finished_processing(&self) -> bool {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.finished_processing,
                    )*
                }
            }

            fn id(&self) -> &str {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.circuit_id,
                    )*
                }
            }

            fn meta(&self) -> &HashMap<String, String> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.meta,
                    )*
                }
            }

            fn num_proofs(&self) -> Option<i32> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.num_proofs,
                    )*
                }
            }

            fn project_name(&self) -> &str {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.project_name,
                    )*
                }
            }

            fn proving_scheme(&self) -> &str {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.proving_scheme,
                    )*
                }
            }

            fn queue_time_sec(&self) -> Option<f64> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => response.queue_time_sec,
                    )*
                }
            }

            fn status(&self) -> &JobStatus {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.status,
                    )*
                }
            }

            fn tags(&self) -> &Vec<String> {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.tags,
                    )*
                }
            }

            fn team(&self) -> &str {
                match self {
                    $(
                        CircuitInfoResponse::$variant(response) => &response.team,
                    )*
                }
            }
        }
    }
}

// Add any new circuit types here
impl_circuit_info!(Boojum, Circom, Halo2, Gnark, Jolt, Noir, Plonky2, Sp1);

/// A wrapper type around [`InternalProofInput`] that provides convenient conversions from
/// various input formats for circuit proofs.
///
/// This type supports automatic conversion from:
/// - String literals (`&str`)
/// - Owned strings (`String`)
/// - JSON values (`serde_json::Value`)
#[derive(Clone, Debug, PartialEq)]
pub struct ProofInput(pub InternalProofInput);

impl From<String> for ProofInput {
    fn from(s: String) -> Self {
        ProofInput(InternalProofInput::String(s))
    }
}

impl From<&str> for ProofInput {
    fn from(s: &str) -> Self {
        ProofInput(InternalProofInput::String(s.to_string()))
    }
}

impl From<serde_json::Value> for ProofInput {
    fn from(v: serde_json::Value) -> Self {
        ProofInput(InternalProofInput::Json(v))
    }
}

// Convert from our wrapper type to the original type
impl From<ProofInput> for InternalProofInput {
    fn from(input: ProofInput) -> Self {
        input.0
    }
}

// Convert from the original type to our wrapper type
impl From<InternalProofInput> for ProofInput {
    fn from(input: InternalProofInput) -> Self {
        ProofInput(input)
    }
}

pub trait ProofInfo {
    fn get_proof_as_serde_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    fn get_proof_as_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

impl ProofInfo for ProofInfoResponse {
    /// Returns the unwrapped proof data as a serde_json::Value
    fn get_proof_as_serde_json(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.proof
            .clone()
            .flatten()
            .ok_or_else(|| "Proof field is not populated".into())
    }

    /// Extracts proof bytes from base64-encoded proof data.
    /// Only applicable for Halo2 and Sp1 circuits.
    fn get_proof_as_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let proof_value = self.get_proof_as_serde_json()?;
        let circuit_type = self.circuit_type;

        // Only proceed if Halo2 or Sp1, where the proof is a single key-value pair
        // in which the value is a base64-encoded string
        match circuit_type {
            CircuitType::Halo2 | CircuitType::Sp1 => {
                let proof_str = extract_single_value(&proof_value)?
                    .as_str()
                    .ok_or("Failed to convert proof to string")?;

                general_purpose::STANDARD
                    .decode(proof_str)
                    .map_err(|e| format!("Failed to decode base64: {}", e).into())
            }
            _ => Err(format!(
                "Proof extraction as bytesnot supported for circuit type: {}",
                circuit_type
            )
            .into()),
        }
    }
}

/// Extracts the value from a JSON object that contains exactly one field.
/// Returns an error if the JSON is not an object or has more/less than one field.
fn extract_single_value(
    value: &serde_json::Value,
) -> Result<&serde_json::Value, Box<dyn std::error::Error>> {
    match value {
        serde_json::Value::Object(map) => {
            if map.len() != 1 {
                return Err("JSON object must have exactly one field".into());
            }
            Ok(map.values().next().unwrap())
        }
        _ => Err("Input must be a JSON object".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_response() -> CircuitInfoResponse {
        CircuitInfoResponse::Noir(Box::new(NoirCircuitInfoResponse {
            circuit_id: "1234".to_string(),
            compute_time_sec: Some(42.5),
            date_created: "2025-01-01".to_string(),
            error: Some("test error".to_string()),
            file_size: Some(1000),
            finished_processing: true,
            meta: HashMap::from([("key".to_string(), "value".to_string())]),
            num_proofs: Some(3),
            project_name: "test_project".to_string(),
            proving_scheme: "groth16".to_string(),
            queue_time_sec: Some(12.3),
            status: JobStatus::Ready,
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            ..Default::default()
        }))
    }

    #[test]
    fn test_circuit_info_getters() {
        let circuit_info = create_test_response();
        assert_eq!(circuit_info.id(), "1234");
        assert_eq!(circuit_info.compute_time_sec(), Some(42.5));
        assert_eq!(circuit_info.date_created(), "2025-01-01");
        assert_eq!(circuit_info.error(), Some("test error".to_string()));
        assert!(circuit_info.finished_processing());
        assert_eq!(
            circuit_info.meta(),
            &HashMap::from([("key".to_string(), "value".to_string())])
        );
        assert_eq!(circuit_info.num_proofs(), Some(3));
        assert_eq!(circuit_info.project_name(), "test_project");
        assert_eq!(circuit_info.proving_scheme(), "groth16");
        assert_eq!(circuit_info.queue_time_sec(), Some(12.3));
        assert_eq!(circuit_info.status(), &JobStatus::Ready);
        assert_eq!(
            circuit_info.tags(),
            &vec!["tag1".to_string(), "tag2".to_string()]
        );
    }

    #[test]
    fn test_proof_input_from_string() {
        let input_string = String::from("x=10\ny=20");
        let proof_input: ProofInput = input_string.clone().into();

        match proof_input.0 {
            InternalProofInput::String(s) => assert_eq!(s, input_string),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_proof_input_from_str() {
        let input_str = "x=10\ny=20";
        let proof_input: ProofInput = input_str.into();

        match proof_input.0 {
            InternalProofInput::String(s) => assert_eq!(s, input_str),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_proof_input_from_json() {
        let json_value = serde_json::json!({
            "x": 10,
            "y": 20
        });
        let proof_input: ProofInput = json_value.clone().into();

        match proof_input.0 {
            InternalProofInput::Json(v) => assert_eq!(v, json_value),
            _ => panic!("Expected JSON variant"),
        }
    }

    #[test]
    fn test_proof_input_roundtrip() {
        // Test ProofInput -> InternalProofInput
        let original = ProofInput(InternalProofInput::String("x=10\ny=20".to_string()));
        let internal: InternalProofInput = original.clone().into();
        match internal.clone() {
            InternalProofInput::String(s) => assert_eq!(s, "x=10\ny=20"),
            _ => panic!("Expected String variant"),
        }

        // Test InternalProofInput -> ProofInput
        let proof_input: ProofInput = internal.into();
        assert_eq!(proof_input, original);
    }

    #[test]
    fn test_proof_into_json_success() {
        let json_value = serde_json::json!({"pi_a": "256", "pi_b": "256"});
        let proof_response = ProofInfoResponse {
            proof: Some(Some(json_value.clone())),
            ..Default::default()
        };

        let result = proof_response.get_proof_as_serde_json().unwrap();
        assert_eq!(result, json_value);
    }

    #[test]
    fn test_proof_into_json_none() {
        let proof_response = ProofInfoResponse {
            proof: None,
            status: JobStatus::Queued,
            ..Default::default()
        };

        let error = proof_response.get_proof_as_serde_json().unwrap_err();
        assert_eq!(error.to_string(), "Proof field is not populated");
    }

    #[test]
    fn test_extract_single_value() {
        // Test successful case
        let json = serde_json::json!({"any_key": "value"});
        let result = extract_single_value(&json).unwrap();
        assert_eq!(result, "value");

        // Test multiple fields
        let json = serde_json::json!({"key1": "value1", "key2": "value2"});
        let error = extract_single_value(&json).unwrap_err();
        assert_eq!(error.to_string(), "JSON object must have exactly one field");

        // Test non-object
        let json = serde_json::json!("not an object");
        let error = extract_single_value(&json).unwrap_err();
        assert_eq!(error.to_string(), "Input must be a JSON object");
    }
}
