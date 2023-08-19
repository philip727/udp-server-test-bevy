use std::net::SocketAddr;
use uuid::Uuid;

pub struct Client {
    socket_address: SocketAddr,
    uuid: Uuid,
}

impl Client {
    pub fn new(socket_address: SocketAddr) -> Self {
        Client {
            socket_address,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
}
