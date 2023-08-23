pub mod constants;
pub mod errors;
pub mod helpers;

use std::{error::Error, fs};

use uuid::Uuid;

use constants::SERVER_DATA_DIR_PATH;

use self::errors::ServerDataError;

pub struct ServerData {
    uuid: Option<Uuid>,
}

impl ServerData {
    pub fn check_for_data_folder() -> Result<(), Box<dyn Error>> {
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

    pub fn check_for_uuid() {
        if let Err(e) = Self::check_for_data_folder() {
            panic!("{}", e)
        }

        let entries = fs::read_dir(SERVER_DATA_DIR_PATH).expect(&format!(
            "Failed to read \'{}\' on server creation",
            SERVER_DATA_DIR_PATH
        ));
    }
}
