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
    /// Run a local league
    Run {
        #[arg(long, default_value = "snake")]
        system: String,

        #[arg(long, default_value_t = 1)]
        matches: usize,

        /// Commit results to persistent ledger
        #[arg(long)]
        commit: bool,

        /// Wallet ID that owns all agents in this run
        #[arg(long)]
        wallet: Option<String>,
    },

    /// Verify an agent against locally committed state
    Verify {
        #[arg(long)]
        agent: String,
    },

    /// Export a commitment for remote verification
    Export {
        #[arg(long)]
        agent: String,

        #[arg(long)]
        out: String,
    },

    /// Verify a commitment independently (remote verifier mode)
    VerifyRemote {
        #[arg(long)]
        commitment: String,

        #[arg(long)]
        agent_wasm: String,
    },

    /// Manually slash an agent (mirrors into wallet)
    Slash {
        #[arg(long)]
        agent: String,

        #[arg(long)]
        amount: u64,
    },

    /// Agent-related commands
    Agent {
        #[command(subcommand)]
        action: AgentCommand,
    },

    /// Show balances for agents and wallets
    Balance {
        #[arg(long)]
        agent: Option<String>,

        #[arg(long)]
        wallet: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum AgentCommand {
    /// Build a guest WASM agent
    Build {
        #[arg(long)]
        path: String,
    },
}
