use bevy::prelude::*;

#[derive(Debug, Component, Default)]
pub struct ServerJoinInput {
    pub server_address: String
}
