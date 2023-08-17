use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod udp_client;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Fullscreen,
                        title: "World Gen Game".to_string(),
                        resizable: false,
                        present_mode: PresentMode::AutoVsync,
                        window_level: WindowLevel::Normal,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
