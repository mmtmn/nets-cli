use std::fs;

use nets::{persist, ledger::Ledger, league_state::LeagueState};
use crate::commands::commitment::{Commitment, SystemParams};

pub fn export(agent: String, out: String) {
    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();

    let state = persist::load("state.json", &mut ledger, &mut league_state);

    let root = state.commitments
        .iter()
        .find(|(id, _)| id == &agent)
        .map(|(_, r)| *r)
        .unwrap_or_else(|| {
            eprintln!("no commitment found for agent {}", agent);
            std::process::exit(1);
        });

    let commitment = Commitment {
        system: "snake".into(),
        agent: agent.clone(),
        merkle_root: hex::encode(root),
        system_params: SystemParams {
            width: 10,
            height: 10,
            steps: 300,
        },
    };

    fs::write(&out, serde_json::to_string_pretty(&commitment).unwrap()).unwrap();
    println!("commitment written to {}", out);
}
