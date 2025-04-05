# Sindri Rust SDK + Web3 Demo

This project demonstrates the first steps towards a ZK web3 gaming application.
In it, a user will play a game of Sudoku in their terminal.
After submitting (or cheating), a user's solution will be checked within a Circom ZK circuit.
This computation (and the resulting zero-knowledge proof) are outsourced to Sindri's GPU proving infrastructure.
After the proof is returned, the local code will simulate an on-chain verification by:
1. Launching `anvil` via [alloy](https://alloy.rs/),
2. Deploying a smart contract verifier,
3. Submitting the verification transaction and proof calldata to the contract.

## Running the Demo

Requirements: 
* [Rustup](https://www.rust-lang.org/tools/install)
* [Foundry](https://book.getfoundry.sh/getting-started/installation) (to launch anvil)
* [Sindri API Key](https://sindri.app/z/me/page/settings/api-keys)

After cloning the project, you can play the game and prove your results via:
```
SINDRI_API_KEY=<your-key-here> cargo run --release
```

## Project Structure

* `circuit/`: Contains the Circom code for checking a sudoku solution
* `contract/`: Contains the Groth16 proof verifier solidity code
* `src/`: Contains the rust package that runs a game, requests a proof, and verifies on-chain
