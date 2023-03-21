mod cli;
mod entities;
mod parser;
mod services;

use clap::Parser;
use cli::Args;
use services::tracking::{punch, TrackError};

use crate::cli::Action;

fn main() -> Result<(), TrackError> {
    let args = Args::parse();

    match args.action {
        Action::Punch => punch(),
        Action::Status => todo!(),
    }
}
