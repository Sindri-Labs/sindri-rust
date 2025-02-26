# Sindri Rust SDK

<img src="./.github/assets/sindri-gradient-logo.webp" height="160" align="right"/>

#### [Sindri Sign Up](https://sindri.app/signup) | [Getting Started](#getting-started) | [Development](#internal-development)

Sindri provides automated ZK proving infrastructure, empowering hundreds of teams—including leading Layer 2s and rollups—to launch in minutes instead of months.
Through the Sindri API, developers can seamlessly integrate verifiable computation, reducing time to market, cutting costs, and scaling faster.
Sindri makes zero-knowledge infrastructure simple and accessible, facilitating automation within the most hardware-intensive layer of the ZK app deployment stack.

This repository contains the Sindri Rust SDK, which provides a client for interacting with the Sindri API.
The [`SindriClient`](./sindri/src/client.rs) struct encapsulates all methods for interacting with the Sindri API.
The client handles request/response management and includes built-in robustness features like automatic retries for transient errors.

# Getting Started

Generate your first zero-knowledge proof in just a few lines of code:

 ```rust
 use sindri_rs::client::SindriClient;

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


## Internal Development

The `scripts/` directory automates updates and testing.

### Updates

The following will grab the newest API spec (downgraded from the actual version to one compatible with openapi-generator) and generate the Sindri API client.
All of the code in `openapi/` is updated with the regeneration, after which `openapi.patch` is applied to the generated client.
Finally, the spec files are removed.

After running this command, inspect the git diff and run tests to ensure whether an SDK update is necessary and whether any manual changes are needed.
```
cd scripts
./update-sdk.sh
```

Note that since the downgraded spec is pulled from a private repo, you need to have a github access token.
You should pass this in as the environment variable `GITHUB_TOKEN`.

### Testing

The `scripts/test-sdk.sh` script has three modes:
* `no-vcr`: Runs the Sindri API client tests without VCR recording/replaying. This is the standard type of test.
* `record`: Runs the Sindri API client tests with VCR recording.  If you run this, a file will be saved in `sindri/tests/recordings/` with every request and response made.
* `replay`: Runs the Sindri API client tests with VCR replaying.  If you run this, the Sindri API client will use the files in `sindri/tests/recordings/` to replay the requests and responses.  This allows us to re-run integration tests without making repetitive calls to the Sindri API.  If any new type of request is made and not found in the recording, the test will fail.

These tests require the environment variables `SINDRI_API_KEY` and `SINDRI_BASE_URL`.
Those are assumed to be set in an `.env` file in the root of this repo.

```
cd scripts
./test-sdk.sh <mode>
```
