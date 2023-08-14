use bevy::prelude::*;

use crate::receiver::events::UdpPacketReceivedEvent;

pub fn handle_messages(mut packet_received_event_writer: EventReader<UdpPacketReceivedEvent>) {
    for received_packet in packet_received_event_writer.iter() {
        info!("hahahaha {:?}", received_packet);
    }
}
