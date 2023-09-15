use bevy::prelude::*;

use super::{
    constants::PACKET_BUFFER_SIZE, events::UdpPacketReceivedEvent, resources::UdpClientManager,
};

pub fn handle_packets(
    udp_client_manager: Res<UdpClientManager>,
    mut packet_received_event_writer: EventWriter<UdpPacketReceivedEvent>,
) {
    let socket = udp_client_manager.get_socket();

    if let Some(socket) = socket {
        let mut buffer = [0u8; PACKET_BUFFER_SIZE];

        match socket.recv_from(&mut buffer) {
            Ok((size, sender_addrs)) => {
                let packet = &buffer[..size];

                info!("Packet Received [{:?}]", packet);

                packet_received_event_writer.send(UdpPacketReceivedEvent {
                    bytes: packet.to_vec(),
                    sender_address: sender_addrs,
                });
            }
            Err(ref e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock) => {}
            Err(e) => {
                info!("Packet Receive Failed [{:?}]", e);
            }
        }
    }
}
