use std::fs;

use nets::fraud::verify_step_proof;

use crate::commands::commitment::{Commitment, FraudProofEnvelope};

pub fn verify_fraud(commitment_path: String, proof_path: String) {
    let commitment_data = fs::read_to_string(&commitment_path)
        .unwrap_or_else(|_| {
            eprintln!("commitment file not found: {}", commitment_path);
            std::process::exit(1);
        });

    let commitment: Commitment =
        serde_json::from_str(&commitment_data).unwrap_or_else(|_| {
            eprintln!("invalid commitment format");
            std::process::exit(1);
        });

    let proof_data = fs::read_to_string(&proof_path)
        .unwrap_or_else(|_| {
            eprintln!("fraud proof file not found: {}", proof_path);
            std::process::exit(1);
        });

    let envelope: FraudProofEnvelope =
        serde_json::from_str(&proof_data).unwrap_or_else(|_| {
            eprintln!("invalid fraud proof format");
            std::process::exit(1);
        });

    let committed_root: [u8; 32] = hex::decode(&commitment.merkle_root)
        .unwrap()
        .try_into()
        .unwrap();

    if verify_step_proof(committed_root, &envelope.proof) {
        println!("VALID FRAUD PROOF: commitment is inconsistent");
    } else {
        eprintln!("INVALID FRAUD PROOF");
        std::process::exit(1);
    }
}
