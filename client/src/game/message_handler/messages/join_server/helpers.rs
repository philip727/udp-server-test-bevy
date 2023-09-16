use std::{
    error::Error,
    path::PathBuf, fs::{self, File}, io,
};

use crate::game::worlds::{helpers::{check_for_world_store_dir, get_stored_worlds}, constants::WORLD_STORE_DIR_PATH};

pub fn get_world_folder(server_id: String) -> Result<Option<PathBuf>, Box<dyn Error>> {
    // Checks if the world store directory exists
    check_for_world_store_dir()?;

    let worlds = get_stored_worlds()?;

    let mut world_folder: Option<PathBuf> = None;

    // Finds the world in the world store
    for world in worlds {
        if let Ok(world) = world {
            let file_path = world.path();

            if file_path.is_file() {
                continue;
            }

            if world.file_name().to_string_lossy().to_string() == server_id {
                world_folder = Some(file_path);
            }
        }
    }

    Ok(world_folder)
}

pub fn create_world_folder(server_id: &str) -> io::Result<()> {
    fs::create_dir(WORLD_STORE_DIR_PATH.to_string() + "/" + server_id)
}

pub fn create_player_data_folder(server_id: &str) -> io::Result<File> {
    File::create(WORLD_STORE_DIR_PATH.to_string() + "/" + server_id + "/" + "player.data")
}
