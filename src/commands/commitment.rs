use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Commitment {
    pub system: String,
    pub agent: String,
    pub merkle_root: String,
    pub system_params: SystemParams,
}

#[derive(Serialize, Deserialize)]
pub struct SystemParams {
    pub width: i32,
    pub height: i32,
    pub steps: u64,
}

#[derive(Serialize, Deserialize)]
pub struct FraudProofEnvelope {
    pub agent: String,
    pub system: String,
    pub committed_root: String,
    pub proof: nets::fraud::TraceStepProof,
}
