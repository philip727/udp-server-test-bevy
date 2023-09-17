use bevy::prelude::*;
use shared::ServerToClientMessage;

use crate::game::message_handler::{
    events::MessageReceivedFromServerEvent, messages::join_server::helpers::get_world_folder,
};

use super::helpers::create_world_folder;

// 
pub fn handle_received_server_id_message(
    mut message_received_event_reader: EventReader<MessageReceivedFromServerEvent>,
) {
    for server_message in message_received_event_reader.iter() {
        match &server_message.message {
            ServerToClientMessage::SendSid(server_id) => {
                let world_folder = get_world_folder(server_id);
                if let Err(e) = world_folder {
                    // show prompt
                    return;
                }

                if let None = world_folder.unwrap() {
                    info!("Creating world folder for [{}]", server_id);
                    let world_folder = create_world_folder(server_id);
                    if let Err(e) = world_folder {
                        // show prompt
                        return;
                    }
                }

                // Request event to get player token and data
            }
            _ => {}
        }
    }
}
