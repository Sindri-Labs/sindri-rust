//! These methods will submit a project deploy or proof request to Sindri,
//! without waiting for the job to complete.

use std::collections::HashMap;

use sindri_openapi::{
    apis::circuits_api::{circuit_create, proof_create},
    models::{CircuitInfoResponse,ProofInfoResponse, ProofInput},
};

use crate::client::SindriClient;

impl SindriClient {

    pub async fn request_build(
        &self,
        project: String,
        tags: Option<Vec<String>>,
        meta: Option<HashMap<String, String>>,
    ) -> Result<CircuitInfoResponse, Box<dyn std::error::Error>> {
        
        Ok(())
    }

    pub async fn request_proof(
        &self,
        circuit_id: &str,
        proof_input: impl Into<ProofInput>,
        meta: Option<HashMap<String, String>>,
        verify: Option<bool>,
        prover_implementation: Option<String>,
    ) -> Result<ProofInfoResponse, Box<dyn std::error::Error>> {
        Ok(())
    }
}
