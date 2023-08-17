pub mod game;
pub mod receiver;

use bevy::{prelude::*, window::ExitCondition};
use game::message_handler::systems::handle_messages;
use receiver::ReceiverPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..Default::default()
            }),
            ReceiverPlugin {
                use_config_file: true,
                ..Default::default()
            },
        ))
        .add_systems(Update, handle_messages)
        .run();
}
