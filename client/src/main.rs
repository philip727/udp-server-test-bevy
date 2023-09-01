use belly::prelude::BellyPlugin;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel},
};

use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::camera::CameraPlugin;
use menu::MenuUIPlugins;
use udp_client::UdpClientPlugin;

pub mod game;
pub mod menu;
pub mod udp_client;

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
            BellyPlugin,
            WorldInspectorPlugin::new(),
            CameraPlugin,
            UdpClientPlugin,
            MenuUIPlugins,
            FramepacePlugin,
        ))
        .add_systems(Startup, setup_app)
        .run();
}

fn setup_app(mut frame_settings: ResMut<FramepaceSettings>) {
    frame_settings.limiter = Limiter::from_framerate(30.0);
}
