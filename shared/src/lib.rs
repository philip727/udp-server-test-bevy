use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    JoinGame
}

impl NetworkMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

