use bevy::prelude::*;

use self::{resources::UdpClientManager, systems::handle_packets, events::UdpPacketReceivedEvent};

pub mod errors;
pub mod resources;
pub mod systems;
pub mod constants;
pub mod events;

pub struct UdpClientPlugin;

impl Plugin for UdpClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UdpClientManager>()
            .add_event::<UdpPacketReceivedEvent>()
            .add_systems(Update, handle_packets);
    }
}
