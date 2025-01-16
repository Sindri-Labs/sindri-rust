# SP1 Proof Example

This example demonstrates how to enable the sindri-rs `sp1-v3` feature to use built-in methods that convert between SP1 structs and Sindri request or response structs.

### Usage
To run within this directory, run 
```SINDRI_API_KEY=<your_api_key> cargo run --release```
You might also find the logs beneficial. To see logs from the SindriClient, you can run with 
```RUST_LOG=debug SINDRI_API_KEY=<your_api_key> cargo run --release```
