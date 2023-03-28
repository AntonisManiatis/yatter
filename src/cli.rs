use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    pub project_path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct PunchArgs {
    /// Path of the target project you want to punch for.
    pub project_path: Option<PathBuf>,
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Debug, Parser)]
pub struct StatusArgs {
    /// Path of the target project you want to punch for.
    pub project_path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Initializes yatter for a project.
    Init(InitArgs),
    /// Inserts a time record in a project based on the system's local time at the time the command was invoked.
    Punch(PunchArgs),
    /// Current tracking status in a project.
    Status(StatusArgs),
}
