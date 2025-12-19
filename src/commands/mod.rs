use crate::cli::{Cli, Command};

pub mod run;
pub mod verify;
pub mod agent;
pub mod balance;

pub fn dispatch(cli: Cli) {
    match cli.command {
        Command::Run {
            system,
            matches,
            commit,
        } => {
            run::run(system, matches, commit);
        }

        Command::Verify { agent } => {
            verify::verify(agent);
        }

        Command::Agent { action } => {
            agent::handle(action);
        }

        Command::Balance { agent } => {
            balance::show(agent);
        }
    }
}
