use std::{fs, error::Error, io::Write};

use bevy::prelude::info;
use uuid::Uuid;

use super::{constants::{SERVER_DATA_DIR_PATH, SERVER_IDENTIFIER_EXTENSION}, errors::ServerDataError};

pub(super) fn check_for_data_folder() -> Result<(), Box<dyn Error>> {
    // Checks if the data directory exists
    if !fs::metadata(SERVER_DATA_DIR_PATH).is_ok()
        || !fs::metadata(SERVER_DATA_DIR_PATH).unwrap().is_dir()
    {
        // Creates the data directory since it doesnt exist
        if let Err(e) = fs::create_dir(SERVER_DATA_DIR_PATH) {
            return Err(Box::new(ServerDataError {
                message: format!(
                    "Failed to create data directory \'{}\': {}",
                    SERVER_DATA_DIR_PATH, e
                ),
            }));
        }
    }

    Ok(())
}

pub(super) fn check_for_sid() -> Option<Uuid> {
    if let Err(e) = check_for_data_folder() {
        panic!("{}", e)
    }

    let entries = fs::read_dir(SERVER_DATA_DIR_PATH).expect(&format!(
        "Failed to read \'{}\' on server creation",
        SERVER_DATA_DIR_PATH
    ));

    // Loops through each entry until we find the .sid
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            if !file_path.is_file() {
                continue;
            }

            let is_extension = file_path.extension();
            if let None = is_extension {
                continue;
            }

            let extension = is_extension.unwrap();
            if extension == SERVER_IDENTIFIER_EXTENSION {
                // Returns the sid found
                return Some(
                    Uuid::parse_str(
                        &file_path
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    )
                    .unwrap(),
                );
            }
        }
    }

    // If no sid is found
    None
}

// Creates a server identifier
pub(super) fn create_sid() -> Result<Uuid, Box<dyn Error>> {
    let uuid = Uuid::new_v4();
    let file_path =
        SERVER_DATA_DIR_PATH.to_owned() + &uuid.to_string() + "." + SERVER_IDENTIFIER_EXTENSION;

    let mut sid_file = fs::File::create(file_path)?;

    sid_file.write_all(b"SERVER IDENTIFIER | DO NOT DELETE THIS")?;

    info!("Successfully created new sid {}.", uuid.to_string());

    Ok(uuid)
}
