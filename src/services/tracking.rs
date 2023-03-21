use crate::{
    entities::TS,
    parser::{ParsingError, ToText},
};

use std::{fs, io::Error, io::ErrorKind, path::PathBuf};

use chrono::{DateTime, Datelike, Local};

/// Name of the directory that holds all the subdirectories for each year.
const DEFAULT_DIR_NAME: &str = "hours";
// TODO: Would be nice if we could find the format the local computer uses for dates and use that.
/// How we format dates of created files.
const FILE_NAME_FORMAT: &str = "%m-%Y";
/// Used in case an OS requires a file extension such as Windows.
const EXPECTED_EXTENSION: &str = "txt";
// TODO: Would be nice if we could find the format the local computer uses for dates and use that.
/// How we formate dates for each day.
const DATE_FORMAT: &str = "%d-%m-%Y";

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
        match err.kind() {
            ErrorKind::NotFound => TrackError::NotFound,
            _ => TrackError::NotSaved,
        }
    }
}

/// Inserts a time record based on the system's local datetime.
pub fn punch() -> Result<(), TrackError> {
    let now = Local::now();

    let dp: PathBuf = [DEFAULT_DIR_NAME, &now.year().to_string()].iter().collect();
    fs::create_dir_all(&dp)?;

    let fp = find_file_for_date(&dp, &now);

    let mut ts;
    if fp.exists() {
        let lines = fs::read_to_string(&fp)?;
        ts = TS::parse(&lines)?
    } else {
        ts = TS::new()
    }

    let today = now.format(DATE_FORMAT).to_string();
    ts.append_entry_for(today);

    // Write the updated TS to the file (creating it if it doesn't exist).
    fs::write(&fp, ts.to_text())?;

    Ok(())
}

fn find_file_for_date(dir_path: &PathBuf, date: &DateTime<Local>) -> PathBuf {
    let file_name = date.format(FILE_NAME_FORMAT).to_string();

    let mut fp = PathBuf::new();
    fp.push(dir_path);
    fp.push(file_name);

    if cfg!(windows) {
        fp.set_extension(EXPECTED_EXTENSION);
    }

    fp
}

#[cfg(test)]
mod tests {}
