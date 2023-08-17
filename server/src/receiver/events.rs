use std::{net::SocketAddr, time::SystemTime};

use bevy::prelude::Event;


#[derive(Event, Debug)]
pub struct UdpPacketReceivedEvent {
    pub packet: Vec<u8>,
    pub sender_address: SocketAddr,
    pub time_received: SystemTime,
}
