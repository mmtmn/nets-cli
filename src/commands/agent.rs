use crate::cli::AgentCommand;

pub fn handle(cmd: AgentCommand) {
    match cmd {
        AgentCommand::Build { path } => {
            println!("building guest agent at path: {}", path);

            // next:
            // - validate crate layout
            // - enforce wasm32-unknown-unknown
            // - run cargo build safely
        }
    }
}
