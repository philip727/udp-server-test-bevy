use std::net::SocketAddr;

use bevy::{utils::HashMap, prelude::*};
use uuid::Uuid;

use super::client::Client;

#[derive(Resource)]
pub struct GameServerManager {
    max_connections: u8,
    connected_clients: HashMap<Uuid, Client>,
    generated_uuids: HashMap<SocketAddr, Uuid>,
}

impl GameServerManager {
    pub fn set_max_connections(&mut self, v: u8) -> &mut Self {
        self.max_connections = v;

        self
    }

    pub fn get_max_connections(&self) -> u8 {
        self.max_connections
    }

    pub fn new_client_from_address(&mut self, socket_address: SocketAddr) {
        let client = Client::new(socket_address);
        let uuid = client.get_uuid();

        self.generated_uuids.insert(socket_address, *uuid);
        self.connected_clients.insert(*uuid, client);
    }

    pub fn get_uuid_from_addrs(&self, socket_address: &SocketAddr) -> Option<&Uuid> {
        self.generated_uuids.get(socket_address)
    }
}
