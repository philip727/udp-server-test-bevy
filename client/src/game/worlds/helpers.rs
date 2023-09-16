use std::{
    error::Error,
    fs::{self, ReadDir}, path::PathBuf,
};

use crate::utils::check_for_dir_folder;

use super::constants::WORLD_STORE_DIR_PATH;

pub fn check_for_world_store_dir() -> Result<(), Box<dyn Error>> {
    if let Err(e) = check_for_dir_folder(WORLD_STORE_DIR_PATH) {
        create_world_store_directory()?;
    }

    Ok(())
}

fn create_world_store_directory() -> Result<(), Box<dyn Error>> {
    fs::create_dir(WORLD_STORE_DIR_PATH)?;

    Ok(())
}

pub fn get_stored_worlds() -> Result<ReadDir, Box<dyn Error>> {
    let entries = fs::read_dir(WORLD_STORE_DIR_PATH)?;
    Ok(entries)
}

pub fn search_for_account_token(server_id: &str, account_token: &str) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(WORLD_STORE_DIR_PATH.to_string() + "/" + server_id)?;
    for entry in entries {
        if let Ok(entry) = entry {

        }
    }


    Ok(None)
}

