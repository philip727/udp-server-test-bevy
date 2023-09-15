use bevy::prelude::*;
use shared::*;

#[derive(Debug, Event)]
pub struct MessageReceivedFromServerEvent {
    pub message: ServerToClientMessage,
}
