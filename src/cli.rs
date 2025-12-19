use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nets")]
#[command(about = "CLI for nets-core competitive agent markets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Run {
        #[arg(long, default_value = "snake")]
        system: String,
        #[arg(long, default_value_t = 1)]
        matches: usize,
        #[arg(long)]
        commit: bool,
        #[arg(long)]
        wallet: Option<String>,
    },

    Verify {
        #[arg(long)]
        agent: String,
    },

    Export {
        #[arg(long)]
        agent: String,
        #[arg(long)]
        out: String,
    },

    VerifyRemote {
        #[arg(long)]
        commitment: String,
        #[arg(long)]
        agent_wasm: String,
    },

    /// Generate a step-level fraud proof
    ProveFraud {
        #[arg(long)]
        commitment: String,
        #[arg(long)]
        agent_wasm: String,
        #[arg(long)]
        out: Option<String>,
        /// Automatically slash the agent if fraud is proven
        #[arg(long)]
        slash: bool,
    },

    /// Verify a step-level fraud proof against a commitment
    VerifyFraud {
        #[arg(long)]
        commitment: String,
        #[arg(long)]
        proof: String,
    },

    Slash {
        #[arg(long)]
        agent: String,
        #[arg(long)]
        amount: u64,
    },

    Agent {
        #[command(subcommand)]
        action: AgentCommand,
    },

    Balance {
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        wallet: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum AgentCommand {
    Build {
        #[arg(long)]
        path: String,
    },
}
