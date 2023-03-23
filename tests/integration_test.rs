use std::{fs, path::Path};

use chrono::{Datelike, Local};
use yatter::{
    self,
    services::tracking::{punch, TrackError},
};

fn init() {
    let path = Path::new("./hours/");
    if path.exists() {
        fs::remove_dir_all("./hours/").unwrap() // whatever
    }
}

#[test]
fn punching_without_providing_any_args_creates_the_hours_dir_in_the_current_working_dir(
) -> Result<(), TrackError> {
    // Arrange
    init();

    // Act
    punch(None)?;

    // Assert
    assert!(Path::new("./hours/").exists());
    Ok(())
}

#[test]
fn providing_a_target_directory_uses_that_as_root_for_the_hours_dir() -> Result<(), TrackError> {
    // Arrange
    let target_project = Path::new("./a_project/");
    let today = Local::now();

    // Act
    punch(Some(target_project.to_path_buf()))?;

    // Assert
    // ? could include the name of the file also but that could be subject to change.
    let p_as_string = format!("./a_project/hours/{}", today.year().to_string());
    assert!(Path::new(&p_as_string).exists());

    // Teardown
    fs::remove_dir_all(target_project)?;

    Ok(())
}
