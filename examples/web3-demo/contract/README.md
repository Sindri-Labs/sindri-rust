# Smart Contract Groth16 Verifier

This smart contract is the standard Groth16 verifier for Circom circuits.
If you use the `sindri-rust/web3-demo` project build, then this smart contract will be compatible with the proofs you produce.

## Developer Notes

1. To regenerate the verifier.json file for use in `src/main.rs`, using the solidity CLI compiler, you would use:
```
solc --optimize Verifier.sol --combined-json abi,bin -o .
jq '.contracts["Verifier.sol:Verifier"]' combined.json > verifier.json
```

2. A Circom verifier contract will have the verification key hard coded in the solidity code! 
   If you recompile the circuit on Sindri without the `zkey` file, make sure you download the new corresponding smart contract (with mock keys) from the Sindri web interface.
