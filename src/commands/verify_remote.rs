use std::fs;

use nets::{
    snake::SnakeSystem,
    match_trace::run_match_with_trace,
};

use crate::commands::{
    commitment::Commitment,
    snake_agent::SnakeWasmAgent,
};

pub fn verify_remote(commitment_path: String, agent_wasm: String) {
    let data = fs::read_to_string(&commitment_path)
        .expect("failed to read commitment file");
    let commitment: Commitment =
        serde_json::from_str(&data).expect("invalid commitment format");

    let wasm = fs::read(&agent_wasm).expect("failed to read agent wasm");

    let mut agent =
        SnakeWasmAgent::load(commitment.agent.clone(), &wasm);

    let system = SnakeSystem::new(
        commitment.system_params.width,
        commitment.system_params.height,
        commitment.system_params.steps,
    );

    let trace = run_match_with_trace(system, &mut agent);
    let root = trace.merkle.root();
    let expected = hex::decode(&commitment.merkle_root)
        .expect("invalid merkle root hex");

    if root[..] != expected[..] {
        eprintln!("FRAUD: merkle root mismatch");
        std::process::exit(1);
    }

    println!("VALID: commitment verified");
}
