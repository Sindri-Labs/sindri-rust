# Sindri Rust SDK Basic Demo

This project demonstrates the fundamentals of using Sindri's Rust SDK for zero-knowledge proof development.

The demo provides a simple example of how to set up and interact with Sindri's proving infrastructure using Rust, enabling you to generate and verify zero-knowledge proofs efficiently using Sindri's GPU-accelerated backend.
In the source code, we define a simple [Noir](https://noir-lang.org/docs/) circuit.
Then in one end-to-end pipeline, we upload that raw circuit code to Sindri and request a proof once the project deployment has succeeded.

## Running the Demo

Requirements: 
* [Rustup](https://www.rust-lang.org/tools/install)
* [Sindri API Key](https://sindri.app/z/me/page/settings/api-keys)

After cloning the project, you can run the demo via:
```
SINDRI_API_KEY=<your-key-here> cargo run --release
```

## Project Structure

* `src/`: Contains the Rust code (`*.rs`) for the demo application as well as the Noir circuit definition (`*.nr`)
* `Cargo.toml`: Cargo manifest describing the prover package
* `Nargo.toml`: Noir manifest describing the circuit package
