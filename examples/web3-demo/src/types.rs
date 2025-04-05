use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// All Circom proofs from Sindri contain the following fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct CircomProof {
    pub pi_a: [U256; 3],
    pub pi_b: [[U256; 2]; 3],
    pub pi_c: [U256; 3],
    pub protocol: String,
}

// A "lite" version of the proof that strips out redundant components.
// All remaining entries in this struct are used as calldata.
#[derive(Debug)]
pub struct CircomProofLite {
    pub pi_a: [U256; 2],
    pub pi_b: [[U256; 2]; 2],
    pub pi_c: [U256; 2],
}

impl CircomProof {
    pub fn to_lite(&self) -> CircomProofLite {
        CircomProofLite {
            pi_a: self.pi_a[..2].try_into().unwrap(),
            pi_b: [
                [self.pi_b[0][1], self.pi_b[0][0]], // Calldata has the order swapped!
                [self.pi_b[1][1], self.pi_b[1][0]], // Calldata has the order swapped!
            ],
            pi_c: self.pi_c[..2].try_into().unwrap(),
        }
    }
}

// Note: the public output from Sindri for a Circom proof depends on the circuit.
// For the Sudoku circuit, this public output is a bit 0/1 conveying the solution
// correctness followed by the original puzzle.
// This function converts the public output response field from ["1", "0", ...]
// to a [U256; 82] array.
pub fn convert_public(value: Value) -> Result<[U256; 82], Box<dyn std::error::Error>> {
    let vec: Vec<String> = serde_json::from_value(value)?;
    let nums: Vec<u8> = vec.iter().map(|s| s.parse()).collect::<Result<_, _>>()?;
    let arr: [u8; 82] = nums
        .try_into()
        .map_err(|_| "Failed to convert Vec to array")?;
    Ok(arr.map(U256::from))
}
