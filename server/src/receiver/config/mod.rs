pub mod errors;
pub mod constants;

use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

use self::{errors::UdpReceiverConfigError, constants::CONFIG_PATH};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct UdpReceiverConfig {
    pub port: Option<u16>,
    pub debugging: Option<bool>
}

impl UdpReceiverConfig {
    pub fn new() -> UdpReceiverConfig {
        UdpReceiverConfig { port: None, debugging: None }
    }

    /// Loads the receiver config from a file located at ["config/server.properties"]
    pub fn from_file() -> Result<UdpReceiverConfig, Box<dyn Error>> {
        let try_open_file = File::open(CONFIG_PATH);

        // Opens the receiver config file and reads the data
        if let Err(_) = try_open_file {
            return Err(Box::new(UdpReceiverConfigError {
                message: "Failed to open udp receiver config file.".to_string(),
            }));
        }

        let mut file = try_open_file.unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Attempts to serialize the file content into the struct
        let try_serialize_json = serde_json::from_str::<UdpReceiverConfig>(&mut content);
        if let Err(e) = try_serialize_json {
            return Err(Box::new(UdpReceiverConfigError {
                message: format!("Failed to load the udp receiver config file. {}", e),
            }));
        }

        Ok(try_serialize_json.unwrap())
    }
}
