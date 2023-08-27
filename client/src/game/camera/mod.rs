use bevy::prelude::{Plugin, Startup};

use self::systems::spawn_camera;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
