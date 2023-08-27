use std::net::SocketAddr;

use bevy::prelude::Event;

use shared::ClientToServerMessage;

#[derive(Debug, Event)]
pub struct MessageReceivedFromClientEvent {
    pub message: ClientToServerMessage,
    pub socket_address: SocketAddr,
}
