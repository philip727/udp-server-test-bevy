use bevy::prelude::*;
use std::{net::*, time::SystemTime};

use super::{
    constants::PACKET_BUFFER_SIZE, events::UdpPacketReceivedEvent, resources::UdpReceiverManager,
    states::UdpReceiverState,
};

pub fn setup_udp_receiver(mut commands: Commands) {
    commands.insert_resource(NextState(Some(UdpReceiverState::Binding)))
}

/// Binds the udp socket for the UDP receiver manager
pub fn bind_udp_socket(
    mut udp_receiver_manager: ResMut<UdpReceiverManager>,
    mut commands: Commands,
) {
    let addrs = udp_receiver_manager
        .bind_address()
        .expect("No bind address was provided for the udp receiver.");

    // Creates the UDP socket
    let socket = UdpSocket::bind(addrs.clone()).expect("Couldn't bind UDP socket to address.");
    udp_receiver_manager.set_socket(socket);

    // Updates the UDP state so it can now handle packets
    info!("Server is now running on {:?}.", addrs);
    commands.insert_resource(NextState(Some(UdpReceiverState::Finished)));
}

pub fn handle_packets(
    udp_receiver_manager: Res<UdpReceiverManager>,
    mut packet_received_event_writer: EventWriter<UdpPacketReceivedEvent>,
) {
    let socket = udp_receiver_manager.socket();

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

                // Packet information
                let packet = &buffer[..size];
                let current_time = SystemTime::now();

                // Write an event with the packet information
                packet_received_event_writer.send(UdpPacketReceivedEvent {
                    packet: packet.to_vec(),
                    sender_address: sender_addrs,
                    time_received: current_time,
                });

                if udp_receiver_manager.debug_mode() {
                    info!(
                        "Packet received [From: {} | Size: {} | Data: {:?}]",
                        sender_addrs, size, packet
                    );
                }
            }
            Err(e) => {
                info!("Packet received but failed [Error: {}]", e);
            }
        }
    }
}
