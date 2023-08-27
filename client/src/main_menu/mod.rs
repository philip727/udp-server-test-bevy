use bevy::prelude::*;

use self::systems::setup_main_menu;

pub mod join_server;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
    }
}
