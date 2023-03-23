use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Parser)]
pub struct PunchArgs {
    /// Path of the target project you want to punch for.
    pub target_project: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Inserts a time record based on the system's local time at the time the command was invoked.
    Punch(PunchArgs),
    /// Current tracking status.
    Status,
}
