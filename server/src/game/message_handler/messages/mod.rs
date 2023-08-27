use bevy::prelude::{Plugin, Update};

use self::join_server::systems::handle_join_server_message;

pub mod join_server;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, handle_join_server_message);
    }
}
