use std::fs;

use nets::{
    fraud::TraceStepProof,
    match_trace::run_match_with_trace,
    snake::SnakeSystem,
};

use crate::commands::{
    commitment::{Commitment, FraudProofEnvelope},
    snake_agent::SnakeWasmAgent,
    fraud_helpers::build_first_divergent_proof,
    slash,
};

pub fn prove_fraud(
    commitment_path: String,
    agent_wasm: String,
    out: Option<String>,
    do_slash: bool,
) {
    let data = fs::read_to_string(&commitment_path)
        .unwrap_or_else(|_| {
            eprintln!("commitment file not found: {}", commitment_path);
            std::process::exit(1);
        });

    let commitment: Commitment =
        serde_json::from_str(&data).unwrap_or_else(|_| {
            eprintln!("invalid commitment format");
            std::process::exit(1);
        });

    let wasm = fs::read(&agent_wasm).unwrap_or_else(|_| {
        eprintln!("agent wasm not found: {}", agent_wasm);
        std::process::exit(1);
    });

    let mut agent = SnakeWasmAgent::load(commitment.agent.clone(), &wasm);

    let system = SnakeSystem::new(
        commitment.system_params.width,
        commitment.system_params.height,
        commitment.system_params.steps,
    );

    let recomputed = run_match_with_trace(system.clone(), &mut agent);

    let committed_root: [u8; 32] = hex::decode(&commitment.merkle_root)
        .unwrap()
        .try_into()
        .unwrap();

    if recomputed.merkle.root() == committed_root {
        eprintln!("no fraud detected: recomputed root matches commitment");
        std::process::exit(1);
    }

    // Build minimal fraud proof
    let proof: TraceStepProof = match build_first_divergent_proof(&recomputed, &recomputed) {
        Some(p) => p,
        None => {
            eprintln!("fraud detected but no divergent step found");
            std::process::exit(1);
        }
    };

    let envelope = FraudProofEnvelope {
        agent: commitment.agent.clone(),
        system: commitment.system.clone(),
        committed_root: commitment.merkle_root.clone(),
        proof,
    };

    let json = serde_json::to_string_pretty(&envelope).unwrap();

    if let Some(path) = out {
        fs::write(&path, &json).unwrap();
        println!("fraud proof written to {}", path);
    } else {
        println!("{}", json);
    }

    if do_slash {
        println!("auto-slashing agent {}", commitment.agent);
        slash::slash(commitment.agent.clone(), 50);
    }
}
