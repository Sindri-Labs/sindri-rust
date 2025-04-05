# Sindri Rust SDK

<img src="https://raw.githubusercontent.com/Sindri-Labs/sindri-rust/refs/heads/main/.github/assets/sindri-gradient-logo.webp" height="160" align="right"/>

#### [Sindri Sign Up](https://sindri.app/signup) | [Getting Started](https://sindri.app/docs/getting-started/) | [Public Projects](https://sindri.app/explore)

Sindri provides automated ZK proving infrastructure, empowering hundreds of teams—including leading Layer 2s and rollups—to launch in minutes instead of months.
Through the Sindri API, developers can seamlessly integrate verifiable computation, reducing time to market, cutting costs, and scaling faster.
Sindri makes zero-knowledge infrastructure simple and accessible, facilitating automation within the most hardware-intensive layer of the ZK app deployment stack.

This repository contains the Sindri Rust SDK, which provides a client for interacting with the Sindri API.
The [`SindriClient`](https://github.com/Sindri-Labs/sindri-rust/blob/main/sindri/src/client.rs) struct encapsulates all methods for interacting with the Sindri API.
The client handles request/response management and includes built-in robustness features like automatic retries for transient errors.

# Getting Started

Generate your first zero-knowledge proof in just a few lines of code:

 ```rust
 use sindri::client::SindriClient;

 let client = SindriClient::new(None, None);
 let circuit = client.create_circuit("path/to/circuit", None, None).await?;
 let input = r#"{"a": 1, "b": 2}"#;
 let proof = client.prove_circuit(circuit.id(), input, None, None, None).await?;
 ```

 # Key Features

- **Project Management**: Create circuits and proofs with methods like
  * `SindriClient.create_circuit()`
  * `SindriClient.prove_circuit()`

- **Collaboration**: Share and reuse public circuits using
  * `SindriClient.clone_circuit()`

- **Seamless Integration**: Works with popular Rust-based DSLs and zkVMs including:
  * [Halo2](https://github.com/axiom-crypto/halo2-lib)
  * [Plonky2](https://github.com/0xPolygonZero/plonky2)
  * [Jolt](https://github.com/a16z/jolt)
  * [Sp1](https://github.com/succinctlabs/sp1)
