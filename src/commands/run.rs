use std::fs;
use std::path::Path;

use nets::{
    agent::Agent,
    snake::SnakeSystem,
    match_trace::run_match_with_trace,
    league::League,
    league_runner::{run_league, LeagueConfig},
    ledger::Ledger,
    evolution::evolve,
    persist,
    league_state::LeagueState,
};

use crate::wallet::mock::MockWalletAdapter;
use crate::commands::snake_agent::SnakeWasmAgent;
use crate::wallet::adapter::WalletAdapter;

/* ------------------------------
   nets run
-------------------------------*/

pub fn run(system: String, matches: usize, commit: bool, wallet: Option<String>) {
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

        let agent_id = path.file_stem().unwrap().to_string_lossy().to_string();
        let wasm = fs::read(&path).expect("failed to read wasm");

        agents.push(SnakeWasmAgent::load(agent_id, &wasm));
    }

    if agents.is_empty() {
        eprintln!("no wasm agents found in agents/");
        std::process::exit(1);
    }

    let system = SnakeSystem::new(10, 10, 300);
    let league = League::bronze();
    let league_cfg = LeagueConfig { matches_per_agent: matches };

    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();

    // Load existing protocol state
    let _ = persist::load("state.json", &mut ledger, &mut league_state);

    // ---- Wallet adapter (CLI-only) ----
    let mut wallet_adapter = MockWalletAdapter::load("state.json");
    let run_wallet = wallet.unwrap_or_else(|| "default".to_string());

    for a in &agents {
        wallet_adapter.bind_agent(&a.id(), &run_wallet);
    }

    let results = run_league(system.clone(), &mut agents, &ledger, &league, &league_cfg);

    println!("\nresults:");
    for r in &results {
        println!("{} total_score={} matches={}", r.agent_id, r.total_score, r.matches.len());
    }

    // ---- Commitments ----
    let mut commitments: Vec<(String, [u8; 32])> = Vec::new();

    println!("\ncommitments:");
    for agent in agents.iter_mut() {
        let trace = run_match_with_trace(system.clone(), agent);
        let root = trace.merkle.root();
        println!("{} merkle_root={:x?}", agent.id(), root);
        commitments.push((agent.id(), root));
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
            wallet_adapter.credit(&r.agent_id, reward);
        }
    }

    if commit {
        persist::save("state.json", &ledger, &league_state, commitments);
        wallet_adapter.save("state.json");
        println!("\nstate + commitments + wallets committed");
    } else {
        println!("\ndry run (use --commit to persist)");
    }
}
