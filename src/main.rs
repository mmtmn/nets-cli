mod cli;
mod commands;
mod wallet;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    commands::dispatch(cli);
}
