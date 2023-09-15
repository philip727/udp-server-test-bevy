use bevy::prelude::*;
use shared::{ClientToServerMessage, MessageToBytes, ServerToClientMessage};

use crate::{
    game::{
        data::resources::ServerDataManager, message_handler::events::MessageReceivedFromClientEvent,
    },
    receiver::resources::UdpReceiverManager,
};

pub fn handle_join_server_message(
    mut message_from_client_event_reader: EventReader<MessageReceivedFromClientEvent>,
    udp_receiver_manager: Res<UdpReceiverManager>,
    server_data_manager: ResMut<ServerDataManager>,
) {
    for client_message in message_from_client_event_reader.iter() {
        match client_message.message {
            ClientToServerMessage::JoinServer => {
                if let Some(socket) = udp_receiver_manager.get_socket() {
                    let sid = server_data_manager.get_server_id().unwrap(); // Server id
                                                                            //
                    let message = ServerToClientMessage::SendSid(sid.to_string())
                        .to_bytes();


                    info!("{:?}", message);

                    // Sends the sid back to the client for them to identify the server
                    if let Err(e) = socket.send_to(message.unwrap().as_slice(), client_message.socket_address) {
                        info!("Error whilst sending message back to the client [{:?}]", e);
                    }
                }
            }
            #[allow(unreachable_patterns)]
            _ => {}
        };
    }
}
