use bevy::prelude::*;
use shared::ServerToClientMessage;

use crate::game::message_handler::{
    events::MessageReceivedFromServerEvent, messages::join_server::helpers::get_world_folder,
};

use super::helpers::create_world_folder;

pub fn handle_received_sid_message(
    mut message_received_event_reader: EventReader<MessageReceivedFromServerEvent>,
) {
    for server_message in message_received_event_reader.iter() {
        match &server_message.message {
            ServerToClientMessage::SendSid(server_id) => {
                let world_folder = get_world_folder(server_id.to_string());
                if let Err(e) = world_folder {
                    // show prompt
                    return;
                }

                match world_folder.unwrap() {
                    Some(file_path) => {
                        info!("Found world folder for [{}]", server_id);
                        // No account token then request

                        // Send token to server
                    }
                    None => {
                        info!("Creating world folder for [{}]", server_id);
                        let world_folder = create_world_folder(server_id);
                        if let Err(e) = world_folder {
                            // show prompt
                            return;
                        }

                        // Request for token
                    }
                }
            }
            _ => {}
        }
    }
}
