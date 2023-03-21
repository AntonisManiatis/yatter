use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Inserts a time record based on the system's local time at the time the command was invoked.
    Punch,
    /// Current tracking status.
    Status,
}
