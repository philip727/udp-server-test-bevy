use bevy::prelude::Plugin;

use self::resources::ServerManager;

pub mod connection_manager;
pub mod data;
pub mod resources;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ServerManager>();
    }
}
