mod cli;

use clap::Parser;
use cli::Args;
use yatter::entities::Punch;
use yatter::services::tracking::{init, punch, status, PunchError, StatusError};

use crate::cli::Action;

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init(args) => {
            match init(args.project_path) {
                Ok(_) => println!("Initialized yatter for project: "), // TODO: add dir?
                Err(init_err) => eprintln!("Error during project initialization! {}", init_err),
            }
        }
        Action::Punch(args) => match punch(args.project_path) {
            Ok(punch) => match punch {
                Punch::In(at) => println!("Punched in {} at {}", "proj", at), // TODO: Add project path
                Punch::Out(at) => println!("Punched out of {} at {}", "proj", at), // TODO: Add project path
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
        Action::Status(args) => match status(args.project_path) {
            Ok(_) => print!("Add a message boss! :D"), // TODO: Add msg.
            Err(status_err) => match status_err {
                StatusError::ProjectNotFound => eprintln!("Error: no project found in {}", ""), // TODO: proj name?
            },
        },
    }
}
