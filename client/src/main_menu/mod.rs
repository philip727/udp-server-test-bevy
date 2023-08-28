use bevy::prelude::*;

use self::systems::{
    handle_exit_button, handle_multiplayer_button, handle_settings_button,
    handle_singleplayer_button, setup_main_menu,
};

pub mod components;
pub mod join_server;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu).add_systems(
            Update,
            (
                handle_singleplayer_button,
                handle_multiplayer_button,
                handle_settings_button,
                handle_exit_button,
            ),
        );
    }
}
