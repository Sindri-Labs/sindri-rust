//! Common types re-exported from the openapi (internal) package.
use std::collections::HashMap;
pub use openapi::models::{
    BoojumCircuitInfoResponse, CircomCircuitInfoResponse, CircuitInfoResponse, CircuitType,
    GnarkCircuitInfoResponse, Halo2CircuitInfoResponse, JobStatus, JoltCircuitInfoResponse,
    NoirCircuitInfoResponse, Plonky2CircuitInfoResponse, ProofInfoResponse, Sp1CircuitInfoResponse,
}; 

/// Helper trait to extract common fields from CircuitInfoResponse
pub trait CircuitInfo {
    fn compute_time_sec(&self) -> Option<f64>;
    fn date_created(&self) -> &str;
    fn error(&self) -> Option<String>;
    fn finished_processing(&self) -> bool;
    fn id(&self) -> &str;
    fn meta(&self) -> &HashMap<String, String>;
    fn num_proofs(&self) -> Option<i32>;
    fn project_name(&self) -> &str;
    fn proving_scheme(&self) -> &str;
    fn queue_time_sec(&self) -> Option<f64>;
    fn status(&self) -> &JobStatus;
    fn tags(&self) -> &Vec<String>;
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
        }
    }
}

// Add any new circuit types here
impl_circuit_info!(
    Boojum, Circom, Halo2, Gnark, Jolt, Noir, Plonky2, Sp1
);

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_response() -> CircuitInfoResponse {
        CircuitInfoResponse::Noir(Box::new(NoirCircuitInfoResponse {
            circuit_id: "1234".to_string(),
            compute_time_sec: Some(42.5),
            date_created: "2025-01-01".to_string(),
            error: Some("test error".to_string()),
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
        assert_eq!(circuit_info.meta(), &HashMap::from([("key".to_string(), "value".to_string())]));
        assert_eq!(circuit_info.num_proofs(), Some(3));
        assert_eq!(circuit_info.project_name(), "test_project");
        assert_eq!(circuit_info.proving_scheme(), "groth16");
        assert_eq!(circuit_info.queue_time_sec(), Some(12.3));
        assert_eq!(circuit_info.status(), &JobStatus::Ready);
        assert_eq!(circuit_info.tags(), &vec!["tag1".to_string(), "tag2".to_string()]);
    }

} 
