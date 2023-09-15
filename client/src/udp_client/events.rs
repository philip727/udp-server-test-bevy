use std::{net::SocketAddr, time::SystemTime};

use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct UdpPacketReceivedEvent {
    pub bytes: Vec<u8>,
    pub sender_address: SocketAddr,
}
