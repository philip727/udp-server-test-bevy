use std::str::Bytes;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientToServerMessage {
    JoinServer
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerToClientMessage {
    SendSid(String),
}

impl MessageToBytes<'_> for ClientToServerMessage {}
impl MessageToBytes<'_> for ServerToClientMessage {}

pub trait MessageToBytes<'message>: Serialize + Sized + Deserialize<'message> {
    fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &'message [u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}
