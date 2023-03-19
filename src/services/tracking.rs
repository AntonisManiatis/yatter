use crate::{entities::TS, parser::ParsingError};

use std::{
    env,
    fs::{self, File},
    io::Error,
    io::ErrorKind,
    path::PathBuf,
};

use chrono::{DateTime, Datelike, Local};

const EXPECTED_EXTENSIONS: [&str; 2] = ["", ".txt"];

/// Name of the directory that holds all the subdirectories for each year.
const DEFAULT_FILE_NAME: &str = "hours";

/// Anything that can go wrong.
#[derive(Debug)]
pub enum TrackError {
    /**
     * sheet not found.
     */
    NotFound,

    /**
     */
    NotParsed, // TODO: This will have extra data in the future.

    /**
     * sheet couldn't be persisted to the underlying storage.
     */
    NotSaved,
}

impl From<ParsingError> for TrackError {
    fn from(_value: ParsingError) -> Self {
        TrackError::NotParsed
    }
}

impl From<Error> for TrackError {
    fn from(err: Error) -> Self {
        // ? Don't know other cases for Errors.

        match err.kind() {
            ErrorKind::NotFound => TrackError::NotFound,
            _ => TrackError::NotSaved,
        }
    }
}

/// Swipes in or out
pub fn swipe() -> Result<(), TrackError> {
    let now = Local::now();
    // TODO: This is specific to Greece's datetime format.
    // TODO: Can we auto detect the format based on the current machine?
    // Here are some crates:
    // - https://crates.io/crates/sys-locale
    // - https://crates.io/crates/chrono_locale
    let date = now.format("%m-%d-%Y").to_string();

    create_dir_if_does_not_exist(&now)?;

    let pb = find_appropriate_file().expect(""); // ? Can I use `?` operator?

    let lines = fs::read_to_string(&pb)?;

    let mut ts = TS::parse(&lines)?;

    ts.append_entry_for(date);

    // Write the updated TS back to the file
    fs::write(&pb, TS::to_text(&ts))?;

    Ok(())
}

fn create_dir_if_does_not_exist(date: &DateTime<Local>) -> std::io::Result<()> {
    let year = date.year().to_string();

    // TODO: this should be windows specific
    let file_name = format!("{}-{}.txt", &year, date.month().to_string());
    let pb: PathBuf = [DEFAULT_FILE_NAME, &year, &file_name].iter().collect();

    fs::create_dir_all(pb)
}

// TODO: There's a better way to find files. check WalkDir crate and also work on arg parsing/file logic a bit.
fn find_appropriate_file() -> Option<PathBuf> {
    let today = Local::now();
    let current_year = today.year();

    // ? Having multiple extensions could cause ambiguities though.
    // ? Can we specify a config val to select which?
    let file_set =
        EXPECTED_EXTENSIONS.map(|ext| format!("{}{}", today.format("%m-%Y").to_string(), ext));

    let root_path = env::current_dir().unwrap(); // TODO: unwrap for now.

    let year_dirs = fs::read_dir(&root_path).expect("root dir doesn't exist");

    let year_path = year_dirs
        .map(|entry| entry.unwrap().path()) // TODO: Unwrap it for now.
        .filter(|path| path.is_dir())
        .filter(|path| path.ends_with(&current_year.to_string())) // TODO: Not ideal, but works :)
        .next();

    // TODO: Current year dir should be created if it doesn't exist.
    if let None = year_path {
        let y_s = current_year.to_string();
        let mut new_path = PathBuf::new();
        new_path.push(&root_path);
        new_path.push(&y_s);
        _ = fs::create_dir(new_path).expect("Failed to create the current year dir.");
    }

    println!("{:?}", year_path);
    let year_paths = fs::read_dir(year_path.unwrap()).unwrap();

    // TODO: Find a file matching the formats we provided.
    // * EG: "03-2023.txt"
    let month_path = year_paths
        .map(|entry| entry.unwrap().path())
        .filter(|pb| pb.ends_with(&PathBuf::from(&file_set[1]))) // TODO: Not ideal, but works :)
        .next();

    println!("{:?}", &month_path);

    if let None = month_path {
        // TODO: Pass in the correct name.
        File::create("./data/2023/1").expect("Failed to create file."); // TODO: Better error
    }

    month_path
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::Local;

    use super::create_dir_if_does_not_exist;

    // #[test]
    fn a_dir_is_created_based_on_the_current_local_date_if_it_does_not_exist() -> Result<()> {
        // Arrange
        let now = Local::now();

        // Act
        create_dir_if_does_not_exist(&now)?;

        // Assert
        todo!()
    }
}
