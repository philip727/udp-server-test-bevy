use std::{error::Error, fs};

use self::errors::OpenFileError;

pub mod errors;

pub fn check_for_dir_folder(dir_path: &str) -> Result<(), Box<dyn Error>> {
    if !fs::metadata(dir_path).is_ok() || !fs::metadata(dir_path).unwrap().is_dir() {
        return Err(OpenFileError::new(format!(
            "Failed to open file at {}",
            dir_path
        )));
    }

    Ok(())
}
