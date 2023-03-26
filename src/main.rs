mod cli;

use clap::Parser;
use cli::Args;
use yatter::entities::Punch;
use yatter::services::tracking::{init, punch, status, PunchError};

use crate::cli::Action;

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init(args) => {
            match init(args.target_project) {
                Ok(_) => println!("Initialized yatter for project: "), // TODO: add dir?
                Err(err) => eprintln!("Error during project initialization! {}", err), // TODO: Better msg.
            }
        }
        Action::Punch(args) => match punch(args.target_project) {
            Ok(punch) => match punch {
                Punch::In(at) => println!("You just punched in {} at {}", "proj", at), // TODO: Add project path
                Punch::Out(at) => println!("You just punched out of {} at {}", "proj", at), // TODO: Add project path
            },
            Err(punch_err) => match punch_err {
                PunchError::ProjectNotFound => eprintln!("Project not found!"), // TODO: add proj path.
                PunchError::NotFound => eprintln!("Time sheet for project {} not found!", ""), // TODO: good to have.
                PunchError::NotParsed => {
                    eprintln!("Time sheet for project {} could not be parsed!", "")
                } // TODO: good to have.
                PunchError::NotSaved => eprintln!("Time sheet for project {} not saved!", ""), // TODO: good to have.
            },
        },
        Action::Status(args) => match status(args.target_project) {
            Ok(_) => println!(""), // TODO: Add msg.
            Err(err) => eprintln!("Error during getting the status in project {}", err),
        },
    }
}
