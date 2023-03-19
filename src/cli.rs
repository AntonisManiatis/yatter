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
    /// Starts tracking.
    Swipe,
    /// Current tracking status.
    Status,
}
