pub mod resources;
pub mod systems;
pub mod constants;
pub mod events;

use bevy::prelude::*;

use self::{resources::UdpReceiverManager, systems::{create_udp_server, handle_packets}, events::UdpPacketReceivedEvent};

pub struct ReceiverPlugin {
    pub socket_address: String,
}

impl ReceiverPlugin {}

impl Plugin for ReceiverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UdpReceiverManager>()
            .add_event::<UdpPacketReceivedEvent>()
            .add_systems(Startup, create_udp_server)
            .add_systems(Update, handle_packets);
    }
}
