use std::net::UdpSocket;

use bevy::prelude::*;
use shared::{ClientToServerMessage, MessageToBytes};

use crate::{
    states::AppState,
    udp_client::resources::UdpClientManager,
};

use super::events::JoinServerEvent;

pub fn handle_sending_join_server_message(
    mut join_server_event_reader: EventReader<JoinServerEvent>,
    mut udp_client_manager: ResMut<UdpClientManager>,
) {
    for event in join_server_event_reader.iter() {
        let server_address = &*event.0;

        let client_socket = UdpSocket::bind("127.0.0.1:0");

        if let Err(..) = client_socket {
            // Send prompt event
            return;
        }

        let client_socket = client_socket.unwrap();

        let attempt_connect = udp_client_manager
            .set_socket(client_socket)
            .connect_to_server(server_address.to_string());

        if let Err(..) = attempt_connect {
            // Send prompt event
            return;
        }

        let _ = attempt_connect
            .unwrap()
            .send_message(&ClientToServerMessage::JoinServer.to_bytes().unwrap());
    }
}

pub fn handle_state_on_join_server(
    mut commands: Commands,
    mut join_server_event_reader: EventReader<JoinServerEvent>,
) {
    for _ in join_server_event_reader.iter() {
        commands.insert_resource(NextState(Some(AppState::Loading)));
    }
}
