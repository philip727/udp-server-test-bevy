use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel},
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::camera::CameraPlugin;
use main_menu::MainMenuPlugin;
use udp_client::UdpClientPlugin;

pub mod udp_client;
pub mod main_menu;
pub mod game;
pub mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Windowed,
                        title: "World Gen Game".to_string(),
                        resizable: true,
                        present_mode: PresentMode::AutoVsync,
                        window_level: WindowLevel::Normal,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            CameraPlugin,
            UdpClientPlugin,
            MainMenuPlugin,
        ))
        .run();
}
