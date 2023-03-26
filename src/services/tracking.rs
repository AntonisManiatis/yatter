use crate::{
    entities::{Punch, TS},
    parser::{ParsingError, ToText},
};

use std::{error, fmt::Display, fs, io::Error, io::ErrorKind, path::PathBuf};

use chrono::{DateTime, Datelike, Local};

/// Name of the directory that holds all the subdirectories for each year.
pub const DEFAULT_DIR_NAME: &str = "hours";
// TODO: Would be nice if we could find the format the local computer uses for dates and use that.
/// How we format dates of created files.
const FILE_NAME_FORMAT: &str = "%m-%Y";
/// Used in case an OS requires a file extension such as Windows.
const EXPECTED_EXTENSION: &str = "txt";
// TODO: Would be nice if we could find the format the local computer uses for dates and use that.
/// How we formate dates for each day.
const DATE_FORMAT: &str = "%d-%m-%Y";

/// Anything that can go wrong while punching.
#[derive(Debug, PartialEq)]
pub enum PunchError {
    // Project not found in the target directory.
    ProjectNotFound,

    /// Sheet not found.
    NotFound,

    /// Sheet could not be parsed.
    NotParsed, // TODO: This will have extra data in the future.

    /// Sheet could not be saved.
    NotSaved,
}

// * I'm implementing error wrapping based on:
// * https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

impl Display for PunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for PunchError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        todo!()
    }
}

impl From<ParsingError> for PunchError {
    fn from(_value: ParsingError) -> Self {
        PunchError::NotParsed
    }
}

impl From<Error> for PunchError {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => PunchError::NotFound,
            _ => PunchError::NotSaved,
        }
    }
}

#[derive(Debug)]
pub enum InitError {}

pub fn init(target_project: Option<PathBuf>) -> Result<(), Error> {
    let now = Local::now();

    let mut dp: PathBuf = PathBuf::new();

    if let Some(tp) = target_project {
        dp.push(tp)
    }

    dp.push(DEFAULT_DIR_NAME);
    dp.push(now.year().to_string());

    if !dp.exists() {
        fs::create_dir_all(&dp)?
    }

    Ok(())
}

/// Inserts a time record based on the system's local datetime.
pub fn punch(target_project: Option<PathBuf>) -> Result<Punch, PunchError> {
    let now = Local::now();

    let mut dp: PathBuf = PathBuf::new();

    if let Some(tp) = target_project {
        dp.push(tp)
    }

    dp.push(DEFAULT_DIR_NAME);
    dp.push(now.year().to_string());

    if !dp.exists() {
        return Err(PunchError::ProjectNotFound);
    }

    let fp = find_file_for_date(&dp, &now);

    let mut ts;
    if fp.exists() {
        let lines = fs::read_to_string(&fp)?;
        ts = TS::parse(&lines)?
    } else {
        ts = TS::new()
    }

    let today = now.format(DATE_FORMAT).to_string();
    ts.append_entry_for(&today);

    let punch = ts.get_last_punch_for(&today);

    // Write the updated TS to the file (creating it if it doesn't exist).
    fs::write(&fp, ts.to_text())?;

    if let Some(punch) = punch {
        return Ok(punch);
    }

    // TODO: Not exactly correct. The line above should NOT fail, we'll re-write this more idiomatically.
    Err(PunchError::NotFound)
}

pub fn status(target_project: Option<PathBuf>) -> Result<(), PunchError> {
    // TODO: Check result.
    todo!()
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
