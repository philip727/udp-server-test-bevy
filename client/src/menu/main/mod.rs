use bevy::prelude::{Plugin, Startup};

use self::systems::setup_main_menu;

pub mod systems;
pub mod helpers;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_main_menu);
    }
}
