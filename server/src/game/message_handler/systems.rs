use bevy::prelude::*;
use shared::*;

use crate::receiver::events::UdpPacketReceivedEvent;

use super::events::MessageReceivedFromClientEvent;

pub fn handle_client_to_server_messages(
    mut packet_received_event_writer: EventReader<UdpPacketReceivedEvent>,
    mut message_received_event_writer: EventWriter<MessageReceivedFromClientEvent>,
) {
    for received_packet in packet_received_event_writer.iter() {
        let message = ClientToServerMessage::from_bytes(&received_packet.bytes);

        info!("Message Received [{:?}]", message);

        if let Ok(message) = message {
            message_received_event_writer.send(MessageReceivedFromClientEvent {
                message,
                socket_address: received_packet.sender_address,
            });
        } else if let Err(e) = message {
            info!("Failed Message [{:?}]", e);
        }
    }
}
