use bevy::prelude::*;

use self::systems::setup_multiplayer_menu;

pub mod components;
pub mod helpers;
pub mod resources;
pub mod systems;

pub struct MultiplayerMenuPlugin;

impl Plugin for MultiplayerMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_multiplayer_menu);
    }
}
