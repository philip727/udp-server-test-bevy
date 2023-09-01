use bevy::prelude::*;

use self::systems::{setup_multiplayer_menu, get_server_address};

pub mod systems;
pub mod components;
pub mod resources;
pub mod helpers;

pub struct MultiplayerMenuPlugin;

impl Plugin for MultiplayerMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_multiplayer_menu)
            .add_systems(Update, get_server_address);
    }
}
