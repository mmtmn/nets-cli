use std::fs;
use std::path::Path;

use nets::{
    system::System,
    agent::Agent,
    snake::{SnakeSystem, Dir},
    wasm_agent::WasmAgent,
    match_trace::run_match_with_trace,
    league::League,
    league_runner::{run_league, LeagueConfig},
    ledger::Ledger,
    evolution::evolve,
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
   nets run
-------------------------------*/

pub fn run(system: String, matches: usize, commit: bool) {
    if system != "snake" {
        eprintln!("only system=snake is supported right now");
        std::process::exit(1);
    }

    let agents_dir = Path::new("agents");
    if !agents_dir.exists() {
        eprintln!("agents/ directory not found");
        std::process::exit(1);
    }

    let mut agents: Vec<SnakeWasmAgent> = Vec::new();

    for entry in fs::read_dir(agents_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }

        let agent_id = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let wasm = fs::read(&path).expect("failed to read wasm");

        let agent = SnakeWasmAgent {
            inner: WasmAgent::load(agent_id.clone(), &wasm)
                .expect("failed to load wasm agent"),
        };

        agents.push(agent);
    }

    if agents.is_empty() {
        eprintln!("no wasm agents found in agents/");
        std::process::exit(1);
    }

    let system = SnakeSystem::new(10, 10, 300);
    let league = League::bronze();
    let league_cfg = LeagueConfig {
        matches_per_agent: matches,
    };

    let mut ledger = Ledger::new();
    let mut league_state = nets::league_state::LeagueState::default();

    persist::load("state.json", &mut ledger, &mut league_state);

    let results =
        run_league(system.clone(), &mut agents, &ledger, &league, &league_cfg);

    println!("\nresults:");
    for r in &results {
        println!(
            "{} total_score={} matches={}",
            r.agent_id,
            r.total_score,
            r.matches.len()
        );
    }

    println!("\ncommitments:");
    for agent in agents.iter_mut() {
        let trace = run_match_with_trace(system.clone(), agent);
        let root = trace.merkle.root();
        println!("{} merkle_root={:x?}", agent.id(), root);
    }

    let mut sorted = results.clone();
    sorted.sort_by(|a, b| b.total_score.cmp(&a.total_score));

    println!("\nsettlement:");
    for (rank, r) in sorted.iter().enumerate() {
        let reward = match rank {
            0 => 100,
            1 => 50,
            _ => 0,
        };

        println!(
            "{} rank={} reward={} capacity_before={}",
            r.agent_id,
            rank + 1,
            reward,
            evolve(&ledger, &r.agent_id)
        );

        if commit {
            ledger.credit(&r.agent_id, reward);
        }
    }

    if commit {
        persist::save("state.json", &ledger, &league_state);
        println!("\nstate committed");
    } else {
        println!("\ndry run (use --commit to persist)");
    }
}
