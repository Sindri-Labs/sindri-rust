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
//! ```ignore
//! use sindri_rs::client::SindriClient;
//!
//! let client = SindriClient::new(None, None);
//! let circuit = client.create_circuit("path/to/circuit", None, None).await?;
//! let input = r#"{"a": 1, "b": 2}"#;
//! let proof = client.prove_circuit(circuit.id(), input, None, None, None).await?;
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
pub(crate) mod utils;

pub mod integrations;
mod types;
pub use types::*;
