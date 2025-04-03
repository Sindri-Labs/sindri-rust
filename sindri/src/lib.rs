//! Sindri Rust SDK
//!
//! A library for generating zero-knowledge proofs with [Sindri](https://sindri.app).
//!
//! # Overview
//!
//! The main entry point of this library is the [`client::SindriClient`] struct, which encapsulates
//! all methods for interacting with the Sindri API. The client handles request/response management
//! and includes built-in robustness features like automatic retries for transient errors.
//!
//! # Quick Start
//!
//! Generate your first zero-knowledge proof in just a few lines of code:
//!
//! ```
//! use std::collections::HashMap;
//! use sindri::{client::SindriClient, CircuitInfo};
//!
//! let client = SindriClient::default();
//! // Use tags and project metadata to organize and annotate your project builds
//! let tags: Option<Vec<String>> = None;
//! let project_metadata: Option<HashMap<String, String>> = None;
//! # let tags = Some(vec!["tester".to_string()]);
//! let circuit = client.create_circuit_blocking(
//!     "../cli/tests/factory/circuit.tar.gz".to_string(),
//!     tags,
//!     project_metadata
//! ).unwrap();
//!
//! // Use proof metadata to annotate proofs with additional information and
//! // pass conditional flags to control whether server-side verification is enabled
//! let input = r#"{"a": 1, "b": 2}"#;
//! let proof_metadata: Option<HashMap<String, String>> = None;
//! let verify: Option<bool> = None;
//! let proof = client.prove_circuit_blocking(
//!     circuit.id(),
//!     input,
//!     proof_metadata,
//!     verify,
//!     None, // The default prover implementation is generally recommended
//! ).unwrap();
//! ```
//!
//! # Key Features
//!
//! - **Project Management**: Create circuits and proofs with methods like
//!   [`create_circuit`](client::SindriClient::create_circuit) and
//!   [`prove_circuit`](client::SindriClient::prove_circuit)
//!
//! - **Collaboration**: Share and reuse public circuits using
//!   [`clone_circuit`](client::SindriClient::clone_circuit)
//!
//! - **Seamless Integration**: Works with popular Rust-based DSLs and zkVMs including:
//!   - Halo2
//!   - Plonky2
//!   - Jolt
//!   - Sp1
//!
//! For more detailed documentation, refer to the [`client::SindriClient`] struct.
//!

pub mod client;

pub(crate) mod custom_middleware;
pub(crate) mod jwt;
pub(crate) mod utils;

pub mod integrations;
mod types;
pub use types::*;

pub mod vendor {
    #[cfg(any(feature = "record", feature = "replay"))]
    pub(crate) mod rvcr;
}
