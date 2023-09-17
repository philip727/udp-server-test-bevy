use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::game::worlds::helpers::{check_for_world_store_dir, get_server_path, get_stored_worlds};

use super::{components::LocalPlayerData, constants::PLAYER_DATA_FILE_NAME};

/// Gets the world directory, returns [None] if it doesn't exist
pub fn get_world_folder(server_id: &str) -> Result<Option<PathBuf>, Box<dyn Error>> {
    // Checks if the world store directory exists
    check_for_world_store_dir()?;

    let path = get_server_path(server_id);

    let mut world_folder: Option<PathBuf> = None;
    if let Ok(metadata) = fs::metadata(path.clone()) {
        if metadata.is_dir() {
            world_folder = Some(path)
        }
    }

    Ok(world_folder)
}

/// Creates world directory
pub fn create_world_folder(server_id: &str) -> io::Result<()> {
    let world_store = fs::create_dir(get_server_path(server_id));
    create_player_data_folder(server_id)?;

    return world_store;
}

/// Creates the player data file in that specific server dir
pub fn create_player_data_folder(server_id: &str) -> io::Result<File> {
    let mut file = File::create(
        get_server_path(server_id).to_str().unwrap().to_string() + "/" + PLAYER_DATA_FILE_NAME,
    )?;

    // Writes default player data to the file
    let data = LocalPlayerData::default();
    let json_str = serde_json::to_string(&data)?;
    file.write_all(json_str.as_bytes())?;

    Ok(file)
}

/// Gets the player data file, returns [None] if doesn't exist
pub fn get_player_data_file(server_id: &str) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let path = PathBuf::from(
        get_server_path(server_id).to_str().unwrap().to_string() + "/" + PLAYER_DATA_FILE_NAME,
    );

    let mut player_data_file: Option<PathBuf> = None;
    if let Ok(metadata) = fs::metadata(path.clone()) {
        if metadata.is_file() {
            player_data_file = Some(path);
        }
    }

    Ok(player_data_file)
}
