use nets::{
    match_trace::MatchTrace,
    fraud::TraceStepProof,
};

/// Find the first step where two traces diverge.
/// Returns None if traces are identical.
pub fn first_divergent_step(a: &MatchTrace, b: &MatchTrace) -> Option<usize> {
    let n = a.steps.len().min(b.steps.len());

    for i in 0..n {
        if a.steps[i].hash() != b.steps[i].hash() {
            return Some(i);
        }
    }

    if a.steps.len() != b.steps.len() {
        return Some(n);
    }

    None
}

/// Build a TraceStepProof for the first divergent step.
/// Returns None if no divergence exists.
pub fn build_first_divergent_proof(
    committed: &MatchTrace,
    recomputed: &MatchTrace,
) -> Option<TraceStepProof> {
    let idx = first_divergent_step(committed, recomputed)?;

    let step = &recomputed.steps[idx];

    Some(TraceStepProof {
        step_index: step.step,
        obs_hash: step.obs_hash,
        action_hash: step.action_hash,
        merkle_path: committed.step_proof(idx),
    })
}
