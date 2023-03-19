mod cli;
mod entities;
mod parser;
mod services;

use clap::Parser;
use cli::Args;
use services::tracking::{swipe, TrackError};

use crate::cli::Action;

fn main() -> Result<(), TrackError> {
    let args = Args::parse();

    match args.action {
        Action::Swipe => swipe(),
        Action::Status => todo!(),
    }
}
