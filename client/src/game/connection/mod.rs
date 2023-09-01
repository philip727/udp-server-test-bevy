use bevy::prelude::{Plugin, Update};

use self::{events::JoinServerEvent, systems::handle_join_server_events};

pub mod events;
pub mod systems;

pub struct HandleConnectionPlugin;

impl Plugin for HandleConnectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<JoinServerEvent>()
            .add_systems(Update, handle_join_server_events);
    }
}
