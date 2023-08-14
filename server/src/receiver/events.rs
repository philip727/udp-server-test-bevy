use std::net::SocketAddr;

use bevy::prelude::Event;


#[derive(Event, Debug)]
pub struct UdpPacketReceivedEvent {
    pub packet: Vec<u8>,
    pub sender_address: SocketAddr,
}
