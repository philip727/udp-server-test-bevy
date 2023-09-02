use bevy::prelude::*;

use self::resources::UdpClientManager;

pub mod errors;
pub mod resources;
pub mod systems;

pub struct UdpClientPlugin;

impl Plugin for UdpClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UdpClientManager>();
    }
}
