mod cli;

use clap::Parser;
use cli::Args;
use yatter::services::tracking::{punch, TrackError};

use crate::cli::Action;

fn main() -> Result<(), TrackError> {
    let args = Args::parse();

    match args.action {
        Action::Punch(args) => punch(args.target_project),
        Action::Status => todo!(),
    }
}
