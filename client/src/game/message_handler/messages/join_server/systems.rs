use bevy::prelude::*;
use shared::ServerToClientMessage;

use crate::game::message_handler::events::MessageReceivedFromServerEvent;

pub fn handle_received_sid_message(
    mut message_received_event_reader: EventReader<MessageReceivedFromServerEvent>,
) {
    for server_message in message_received_event_reader.iter() {
        match &server_message.message {
            ServerToClientMessage::SendSid(server_id) => {
                info!("{}", server_id);
            }
            _ => {}
        }
    }
}
