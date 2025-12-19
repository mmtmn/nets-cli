use crate::cli::{Cli, Command};

pub mod run;
pub mod verify;
pub mod export;
pub mod verify_remote;
pub mod slash;
pub mod agent;
pub mod balance;

pub mod snake_agent;
pub mod commitment;

pub fn dispatch(cli: Cli) {
    match cli.command {
        Command::Run { system, matches, commit, wallet } => {
            run::run(system, matches, commit, wallet);
        }

        Command::Verify { agent } => {
            verify::verify(agent);
        }

        Command::Export { agent, out } => {
            export::export(agent, out);
        }

        Command::VerifyRemote { commitment, agent_wasm } => {
            verify_remote::verify_remote(commitment, agent_wasm);
        }

        Command::Slash { agent, amount } => {
            slash::slash(agent, amount);
        }

        Command::Agent { action } => {
            agent::handle(action);
        }

        Command::Balance { agent, wallet } => {
            balance::show(agent, wallet);
        }
    }
}
