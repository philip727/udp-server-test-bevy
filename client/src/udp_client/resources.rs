use std::{net::UdpSocket, error::Error};

use super::errors::UdpClientError;

pub struct UdpClientManager {
    socket: Option<UdpSocket>,
}

impl Default for UdpClientManager {
    fn default() -> Self {
        UdpClientManager { socket: None }
    }
}

impl UdpClientManager {
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

    /// Connects the udp socket to that server address
    pub fn connect_to_server(&mut self, server_address: String) -> Result<(), Box<dyn Error>> {
        self.socket?.connect(server_address)?;

        Ok(())
    }

    /// Sends a messages to the connected server address
    pub fn send_message(&self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.socket?.send(buf);

        Ok(())
    }

}
