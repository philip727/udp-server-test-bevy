use bevy::prelude::*;
use std::net::*;

use super::{
    constants::PACKET_BUFFER_SIZE, events::UdpPacketReceivedEvent, resources::UdpReceiverManager,
};

pub fn create_udp_server(mut udp_server: ResMut<UdpReceiverManager>) {
    let addrs = "127.0.0.1:35642";
    let socket = UdpSocket::bind(addrs).expect("Couldn't bind UDP socket to address.");
    udp_server.set_socket(socket);

    info!("Server is now running on {:?}.", addrs);
}

pub fn handle_packets(
    udp_server: Res<UdpReceiverManager>,
    mut packet_received_event_writer: EventWriter<UdpPacketReceivedEvent>,
) {
    let socket = udp_server.socket();

    if let Some(socket) = socket {
        let mut buffer = [0; PACKET_BUFFER_SIZE];
        match socket.recv_from(&mut buffer) {
            Ok((size, sender_addrs)) => {
                // Cancel the packet if it was too large
                if size > PACKET_BUFFER_SIZE {
                    info!(
                        "Packet received but was too large [Size: {} | Max Size: {}]",
                        size, PACKET_BUFFER_SIZE
                    );
                    return;
                }

                let packet = &buffer[..size];

                // Write an event with the packet information
                packet_received_event_writer.send(UdpPacketReceivedEvent {
                    packet: packet.to_vec(),
                    sender_address: sender_addrs,
                });

                info!(
                    "Packet received [From: {} | Size: {} | Data: {:?}]",
                    sender_addrs, size, packet
                );
            }
            Err(_) => {}
        }
    }
}
