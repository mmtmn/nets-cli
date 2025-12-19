use std::fs;
use std::path::Path;

use nets::{
    system::System,
    agent::Agent,
    snake::{SnakeSystem, Dir},
    wasm_agent::WasmAgent,
    match_trace::run_match_with_trace,
    ledger::Ledger,
    league_state::LeagueState,
    persist,
};

/* ------------------------------
   Action decoding
-------------------------------*/

fn u64_to_dir(v: u64) -> Dir {
    match v % 4 {
        0 => Dir::Up,
        1 => Dir::Down,
        2 => Dir::Left,
        _ => Dir::Right,
    }
}

/* ------------------------------
   WASM Snake Adapter
-------------------------------*/

struct SnakeWasmAgent {
    inner: WasmAgent,
}

impl Agent<
    <SnakeSystem as System>::Observation,
    <SnakeSystem as System>::Action,
> for SnakeWasmAgent {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn decide(
        &mut self,
        obs: <SnakeSystem as System>::Observation,
    ) -> <SnakeSystem as System>::Action {
        let ((hx, hy), (ax, ay), _) = obs;

        let packed =
            ((hx as u64 & 0xFF) << 24)
            | ((hy as u64 & 0xFF) << 16)
            | ((ax as u64 & 0xFF) << 8)
            | (ay as u64 & 0xFF);

        u64_to_dir(self.inner.decide(packed))
    }
}

/* ------------------------------
   nets verify (commitment-aware)
-------------------------------*/

pub fn verify(agent: String) {
    let agents_dir = Path::new("agents");
    let wasm_path = agents_dir.join(format!("{}.wasm", agent));

    if !wasm_path.exists() {
        eprintln!("agent wasm not found: {:?}", wasm_path);
        std::process::exit(1);
    }

    let wasm = fs::read(&wasm_path).expect("failed to read wasm");

    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();
    let state = persist::load("state.json", &mut ledger, &mut league_state);

    let expected_root = state
        .commitments
        .iter()
        .find(|(id, _)| id == &agent)
        .map(|(_, root)| *root)
        .unwrap_or_else(|| {
            eprintln!("no committed root found for agent {}", agent);
            std::process::exit(1);
        });

    let mut agent_instance = SnakeWasmAgent {
        inner: WasmAgent::load(agent.clone(), &wasm)
            .expect("failed to load agent"),
    };

    let system = SnakeSystem::new(10, 10, 300);
    let trace = run_match_with_trace(system, &mut agent_instance);
    let recomputed_root = trace.merkle.root();

    if recomputed_root != expected_root {
        eprintln!(
            "verification failed: merkle root mismatch\nexpected={:x?}\nactual={:x?}",
            expected_root, recomputed_root
        );
        std::process::exit(1);
    }

    println!(
        "verification ok: agent={} merkle_root={:x?}",
        agent, recomputed_root
    );
}
