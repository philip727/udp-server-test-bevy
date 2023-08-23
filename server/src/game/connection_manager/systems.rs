use bevy::prelude::*;
use shared::NetworkMessage;

use crate::receiver::events::UdpPacketReceivedEvent;

pub fn handle_messages(mut packet_received_event_writer: EventReader<UdpPacketReceivedEvent>) {
    for received_packet in packet_received_event_writer.iter() {
        let message = NetworkMessage::from_bytes(&received_packet.bytes);
        info!("{:?}", message);
        if let Ok(message) = message {
            match message {
                NetworkMessage::JoinGame => {

                }
            }
        }
    }
}
