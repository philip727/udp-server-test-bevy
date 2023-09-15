use bevy::prelude::*;

use shared::*;

use super::events::MessageReceivedFromServerEvent;
use crate::udp_client::events::UdpPacketReceivedEvent;

pub fn handle_server_to_client_messages(
    mut packet_received_event_reader: EventReader<UdpPacketReceivedEvent>,
    mut message_received_event_writer: EventWriter<MessageReceivedFromServerEvent>,
) {
    for received_packet in packet_received_event_reader.iter() {
        let message = ServerToClientMessage::from_bytes(&received_packet.bytes);
        info!("Message Received [{:?}]", message);

        if let Ok(message) = message {
            message_received_event_writer.send(MessageReceivedFromServerEvent { message })
        } else if let Err(e) = message {
            info!("Failed Message [{:?}]", e);
        }
    }
}
