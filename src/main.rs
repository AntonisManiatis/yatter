mod cli;

use clap::Parser;
use cli::Args;
use yatter::entities::Punch;
use yatter::services::tracking::{punch, TrackError};

use crate::cli::Action;

fn main() -> Result<(), TrackError> {
    let args = Args::parse();

    match args.action {
        Action::Punch(args) => {
            if let Ok(punch) = punch(args.target_project) {
                match punch {
                    // TODO: An enhancement would be to also print the project.
                    Punch::In(at) => println!("You punched in at {}", at),
                    Punch::Out(at) => println!("You punched out at {}", at),
                }
            }
        }
        Action::Status => todo!(),
    }

    Ok(())
}
