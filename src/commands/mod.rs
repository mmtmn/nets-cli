use crate::cli::{Cli, Command};

pub mod run;
pub mod verify;
pub mod export;
pub mod verify_remote;
pub mod agent;
pub mod balance;

pub mod snake_agent;
pub mod commitment;

pub fn dispatch(cli: Cli) {
    match cli.command {
        Command::Run { system, matches, commit } => {
            run::run(system, matches, commit);
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

        Command::Agent { action } => {
            agent::handle(action);
        }

        Command::Balance { agent } => {
            balance::show(agent);
        }
    }
}
