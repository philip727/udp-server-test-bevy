use std::net::UdpSocket;

use bevy::prelude::*;

#[derive(Resource)]
pub struct UdpReceiverManager {
    socket: Option<UdpSocket>,
}

impl Default for UdpReceiverManager {
    fn default() -> Self {
        UdpReceiverManager {
            socket: None,
        }
    }
}

impl UdpReceiverManager {
    pub fn set_socket(&mut self, socket: UdpSocket) -> &mut Self {
        self.socket = Some(socket);

        self
    }

    pub fn socket(&self) -> Option<&UdpSocket> {
        self.socket.as_ref()
    }

    pub fn mut_socket(&mut self) -> Option<&mut UdpSocket> {
        self.socket.as_mut()
    }
}
