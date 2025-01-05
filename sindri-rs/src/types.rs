//! Common types re-exported from the openapi (internal) package.

pub use openapi::models::{
    BoojumCircuitInfoResponse, CircomCircuitInfoResponse, CircuitInfoResponse, CircuitType,
    GnarkCircuitInfoResponse, Halo2CircuitInfoResponse, JobStatus, JoltCircuitInfoResponse,
    NoirCircuitInfoResponse, Plonky2CircuitInfoResponse, ProofInfoResponse, Sp1CircuitInfoResponse,
}; 

/// Helper trait to extract common fields from CircuitInfoResponse
pub trait CircuitInfo {
    /// Get the circuit ID from any CircuitInfoResponse variant
    fn circuit_id(&self) -> &str;
}

impl CircuitInfo for CircuitInfoResponse {
    fn circuit_id(&self) -> &str {
        match self {
            CircuitInfoResponse::Boojum(response) => &response.circuit_id,
            CircuitInfoResponse::Circom(response) => &response.circuit_id,
            CircuitInfoResponse::Halo2(response) => &response.circuit_id,
            CircuitInfoResponse::Gnark(response) => &response.circuit_id,
            CircuitInfoResponse::Jolt(response) => &response.circuit_id,
            CircuitInfoResponse::Noir(response) => &response.circuit_id,
            CircuitInfoResponse::Plonky2(response) => &response.circuit_id,
            CircuitInfoResponse::Sp1(response) => &response.circuit_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_id_getter() {
        let circuit_info = CircuitInfoResponse::Noir(Box::new(NoirCircuitInfoResponse {
            circuit_id: "1234".to_string(),
            ..Default::default()
        }));
        
        assert_eq!(circuit_info.circuit_id(), "1234");
    }
} 