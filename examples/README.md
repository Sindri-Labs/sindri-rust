# Sindri Rust SDK Examples

This directory contains example code demonstrating various use cases for the `sindri` SDK.

## Examples Overview

The examples in this directory showcase how to:

- Deploy and prove zero-knowledge circuits using Sindri
- Download public circuits to work with a local copy
- Generate proofs for your circuits
- Retrieve proofs from Sindri
- Verify proofs locally

## Prerequisites

You will need to obtain an API key to use Sindri.
If you have not already created an account, you can do so by visiting the [Sindri sign up page](https://sindri.app/signup).
After logging into the [Sindri front-end](https://sindri.app/login), you can create and manage your API Keys within the [API Keys Settings page](https://sindri.app/z/me/page/settings/api-keys).

Once you have an API key, you can set it as the `SINDRI_API_KEY` environment variable:
```bash
export SINDRI_API_KEY=<your_api_key>
```


## Running the Examples

Each example can be run from the root of the repository using:
```bash
cargo run -p <example_name>
```
For extra logging, you can run with the `RUST_LOG` environment variable set to `log` or `debug`.

## Available Examples

- Create a plonk proof via the Sp1 zkVM from public guest code and verify it locally:[sp1-proof](sp1-proof/README.md)

## Additional Resources

- [Sindri Documentation](https://sindri.app/docs/)
