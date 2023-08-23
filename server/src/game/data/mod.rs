pub mod constants;
pub mod errors;
pub mod helpers;

use std::{error::Error, fs, io::Write};

use bevy::prelude::*;
use uuid::Uuid;

use constants::SERVER_DATA_DIR_PATH;

use self::{constants::SERVER_IDENTIFIER_EXTENSION, errors::ServerDataError};

pub struct ServerData {
    uuid: Uuid,
}

impl ServerData {
    fn check_for_data_folder() -> Result<(), Box<dyn Error>> {
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

    fn check_for_sid() -> Option<Uuid> {
        if let Err(e) = Self::check_for_data_folder() {
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
                    return Some(
                        // Returns the sid found
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

        // If no sid is found then we return
        None
    }

    fn create_sid() -> Result<Uuid, Box<dyn Error>> {
        let uuid = Uuid::new_v4();
        let file_path =
            SERVER_DATA_DIR_PATH.to_owned() + &uuid.to_string() + "." + SERVER_IDENTIFIER_EXTENSION;

        let mut sid_file = std::fs::File::create(file_path)?;

        sid_file.write_all(b"DO NOT DELETE THIS")?;

        info!(
            "Successfully created new sid {}",
            uuid.to_string()
        );

        Ok(uuid)
    }

    pub fn new() -> ServerData {
        let mut has_uuid = Self::check_for_sid();
        if let None = has_uuid {
            has_uuid =
                Some(Self::create_sid().expect("Failed to create sid, please try running again."));
        }

        info!("Found sid {}", has_uuid.unwrap());

        ServerData {
            uuid: has_uuid.unwrap(),
        }
    }
}
