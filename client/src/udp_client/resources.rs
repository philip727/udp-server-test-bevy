use std::net::UdpSocket;

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

    pub fn send_message(&self) -> Result<> {
    }

}
