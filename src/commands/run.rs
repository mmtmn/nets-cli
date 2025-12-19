use std::fs;
use std::path::Path;

use nets::{
    agent::Agent,
    match_trace::run_match_with_trace,
    league::League,
    league_runner::{run_league, LeagueConfig},
    ledger::Ledger,
    evolution::evolve,
    persist,
    league_state::LeagueState,

    snake::SnakeSystem,
    chess::ChessSystem,
    rps::RpsSystem,
};

use crate::commands::snake_agent::SnakeWasmAgent;
use crate::commands::chess_agent::ChessWasmAgent;
use crate::commands::rps_agent::RpsWasmAgent;

use crate::wallet::mock::MockWalletAdapter;
use crate::wallet::adapter::WalletAdapter;

pub fn run(system: String, matches: usize, commit: bool, wallet: Option<String>) {
    let agents_dir = Path::new("agents");
    if !agents_dir.exists() {
        eprintln!("agents/ directory not found");
        std::process::exit(1);
    }

    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();
    persist::load("state.json", &mut ledger, &mut league_state);

    let mut wallet_adapter = MockWalletAdapter::load("state.json");
    let run_wallet = wallet.unwrap_or_else(|| "default".into());

    let league = League::bronze();
    let league_cfg = LeagueConfig { matches_per_agent: matches };

    match system.as_str() {
        /* =======================
           SNAKE
        ======================= */
        "snake" => {
            let mut agents = Vec::new();

            for entry in fs::read_dir(agents_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                    continue;
                }

                let id = path.file_stem().unwrap().to_string_lossy().to_string();
                let wasm = fs::read(&path).unwrap();

                wallet_adapter.bind_agent(&id, &run_wallet);
                agents.push(SnakeWasmAgent::load(id, &wasm));
            }

            let system = SnakeSystem::new(10, 10, 300);
            let results = run_league(system.clone(), &mut agents, &ledger, &league, &league_cfg);

            println!("\nresults:");
            for r in &results {
                println!(
                    "{} total_score={} matches={}",
                    r.agent_id,
                    r.total_score,
                    r.matches.len()
                );
            }

            let mut commitments = Vec::new();
            println!("\ncommitments:");
            for agent in agents.iter_mut() {
                let trace = run_match_with_trace(system.clone(), agent);
                let root = trace.merkle.root();
                println!("{} merkle_root={:x?}", agent.id(), root);
                commitments.push((agent.id(), root));
            }

            settle_and_persist(
                results,
                &mut ledger,
                &mut wallet_adapter,
                &mut league_state,
                commitments,
                commit,
            );
        }

        /* =======================
           CHESS
        ======================= */
        "chess" => {
            let mut agents = Vec::new();

            for entry in fs::read_dir(agents_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                    continue;
                }

                let id = path.file_stem().unwrap().to_string_lossy().to_string();
                let wasm = fs::read(&path).unwrap();

                wallet_adapter.bind_agent(&id, &run_wallet);
                agents.push(ChessWasmAgent::load(id, &wasm));
            }

            let system = ChessSystem::new(200);
            let results = run_league(system.clone(), &mut agents, &ledger, &league, &league_cfg);

            println!("\nresults:");
            for r in &results {
                println!(
                    "{} total_score={} matches={}",
                    r.agent_id,
                    r.total_score,
                    r.matches.len()
                );
            }

            let mut commitments = Vec::new();
            println!("\ncommitments:");
            for agent in agents.iter_mut() {
                let trace = run_match_with_trace(system.clone(), agent);
                let root = trace.merkle.root();
                println!("{} merkle_root={:x?}", agent.id(), root);
                commitments.push((agent.id(), root));
            }

            settle_and_persist(
                results,
                &mut ledger,
                &mut wallet_adapter,
                &mut league_state,
                commitments,
                commit,
            );
        }

        /* =======================
           ROCK PAPER SCISSORS
        ======================= */
        "rps" => {
            let mut agents = Vec::new();

            for entry in fs::read_dir(agents_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                    continue;
                }

                let id = path.file_stem().unwrap().to_string_lossy().to_string();
                let wasm = fs::read(&path).unwrap();

                wallet_adapter.bind_agent(&id, &run_wallet);
                agents.push(RpsWasmAgent::load(id, &wasm));
            }

            let system = RpsSystem::new(100);
            let results = run_league(system.clone(), &mut agents, &ledger, &league, &league_cfg);

            println!("\nresults:");
            for r in &results {
                println!(
                    "{} total_score={} matches={}",
                    r.agent_id,
                    r.total_score,
                    r.matches.len()
                );
            }

            let mut commitments = Vec::new();
            println!("\ncommitments:");
            for agent in agents.iter_mut() {
                let trace = run_match_with_trace(system.clone(), agent);
                let root = trace.merkle.root();
                println!("{} merkle_root={:x?}", agent.id(), root);
                commitments.push((agent.id(), root));
            }

            settle_and_persist(
                results,
                &mut ledger,
                &mut wallet_adapter,
                &mut league_state,
                commitments,
                commit,
            );
        }

        _ => {
            eprintln!("unknown system '{}'", system);
            std::process::exit(1);
        }
    }
}

/* ------------------------------
   Settlement (non-generic)
-------------------------------*/

fn settle_and_persist<S: nets::system::System>(
    mut results: Vec<nets::league_runner::LeagueResult<S>>,
    ledger: &mut Ledger,
    wallet_adapter: &mut MockWalletAdapter,
    league_state: &mut LeagueState,
    commitments: Vec<(String, [u8; 32])>,
    commit: bool,
) {
    results.sort_by(|a, b| b.total_score.cmp(&a.total_score));

    println!("\nsettlement:");
    for (rank, r) in results.iter().enumerate() {
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
            evolve(ledger, &r.agent_id)
        );

        if commit {
            ledger.credit(&r.agent_id, reward);
            wallet_adapter.credit(&r.agent_id, reward);
        }
    }

    if commit {
        persist::save("state.json", ledger, league_state, commitments);
        wallet_adapter.save("state.json");
        println!("\nstate + commitments + wallets committed");
    }
}
