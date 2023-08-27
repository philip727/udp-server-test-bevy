use std::net::UdpSocket;

use bevy::prelude::*;
use shared::{ClientToServerMessage, MessageToBytes};

use self::resources::UdpClientManager;

pub mod errors;
pub mod resources;
pub mod systems;

pub struct UdpClientPlugin;

impl Plugin for UdpClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UdpClientManager>();
    }
}

// client example
pub fn test_system(mut udp_client_manager: ResMut<UdpClientManager>) {
    let client_socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let _ = udp_client_manager
        .set_socket(client_socket)
        .connect_to_server(String::from("127.0.0.1:32695"))
        .unwrap()
        .send_message(&ClientToServerMessage::JoinServer.to_bytes().unwrap());
}
