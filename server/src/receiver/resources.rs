use std::net::UdpSocket;

use bevy::prelude::*;

#[derive(Resource)]
pub struct UdpReceiverManager {
    bind_address: Option<String>,
    socket: Option<UdpSocket>,
    debug_mode: bool
}

impl Default for UdpReceiverManager {
    fn default() -> Self {
        UdpReceiverManager {
            bind_address: None,
            socket: None,
            debug_mode: true
        }
    }
}

impl UdpReceiverManager {
    pub fn set_socket(&mut self, socket: UdpSocket) -> &mut Self {
        self.socket = Some(socket);

        self
    }

    pub fn set_bind_address(&mut self, addrs: String) -> &mut Self {
        self.bind_address = Some(addrs);

        self
    }

    pub fn set_debug_mode(&mut self, enabled: bool) -> &mut Self {
        self.debug_mode = enabled;

        self
    }

    /// Is the UDP receiver is in debug mode
    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }

    pub fn bind_address(&self) -> Option<String> {
        self.bind_address.clone()
    }

    // Gets the udp socket
    pub fn get_socket(&self) -> Option<&UdpSocket> {
        self.socket.as_ref()
    }

    // Gets a mutable reference to the udp socket
    pub fn mut_socket(&mut self) -> Option<&mut UdpSocket> {
        self.socket.as_mut()
    }
}
