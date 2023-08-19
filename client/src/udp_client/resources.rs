use std::{error::Error, net::UdpSocket};

use bevy::prelude::*;

use super::errors::UdpClientError;

#[derive(Resource)]
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

    pub fn get_socket(&self) -> Option<&UdpSocket> {
        self.socket.as_ref()
    }

    pub fn mut_socket(&mut self) -> Option<&mut UdpSocket> {
        self.socket.as_mut()
    }

    /// Connects the udp socket to that server address
    pub fn connect_to_server(&mut self, server_address: String) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(socket) = &self.socket {
            socket.connect(server_address)?;

            return Ok(self);
        }

        Err(Box::new(UdpClientError {
            message: String::from(format!(
                "The client failed to connect to {}.",
                server_address
            )),
        }))
    }

    /// Sends a messages to the connected server address
    pub fn send_message(&self, buf: &[u8]) -> Result<&Self, Box<dyn Error>> {
        if let Some(socket) = &self.socket {
            socket.send(buf)?;

            return Ok(self);
        }

        Err(Box::new(UdpClientError {
            message: String::from("Failed to send a network message."),
        }))
    }
}
