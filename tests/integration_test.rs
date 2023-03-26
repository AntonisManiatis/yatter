use std::{fs, path::Path};

use chrono::{Datelike, Local};
use yatter::{
    self,
    services::tracking::{init, punch, PunchError, DEFAULT_DIR_NAME},
};

#[test]
fn punching_in_a_directory_that_is_not_initialized_returns_a_project_not_found_error() {
    // Arrange
    let project_dir = Path::new("./this_project_does_not_exist");

    // Act
    match punch(Some(project_dir.to_path_buf())) {
        // Assert
        Ok(_) => panic!(),
        Err(punch_error) => {
            assert_eq!(PunchError::ProjectNotFound, punch_error);
        }
    }
}

#[test]
fn init_creates_a_directory_in_the_specified_path_if_it_not_exists() {
    // Arrange
    let project_dir = Path::new("./new_project");

    // Act
    let result = init(Some(project_dir.to_path_buf()));

    // Assert
    assert!(result.is_ok());

    // Teardown
    fs::remove_dir_all(project_dir).unwrap();
}

#[test]
fn punching_without_providing_any_args_creates_the_hours_dir_in_the_current_working_dir(
) -> Result<(), PunchError> {
    // Arrange
    let dir: String = format!("./{}/", DEFAULT_DIR_NAME);
    init(None)?;

    // Act
    punch(None)?;

    // Assert
    assert!(Path::new(&dir).exists());
    Ok(())
}

#[test]
fn providing_a_target_directory_uses_that_as_root_for_the_hours_dir() -> Result<(), PunchError> {
    // Arrange
    let target_project = Path::new("./a_project/");
    let today = Local::now();

    init(Some(target_project.to_path_buf()))?;

    // Act
    punch(Some(target_project.to_path_buf()))?;

    // Assert
    let p_as_string = format!(
        "./a_project/{}/{}",
        DEFAULT_DIR_NAME,
        today.year().to_string()
    );

    assert!(Path::new(&p_as_string).exists());

    // Teardown
    fs::remove_dir_all(target_project)?;

    Ok(())
}
