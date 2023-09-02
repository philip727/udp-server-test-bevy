use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};

use crate::states::AppState;

use self::{events::JoinServerEvent, systems::handle_connection_on_join_server};

pub mod events;
pub mod systems;

pub struct HandleConnectionPlugin;

impl Plugin for HandleConnectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<JoinServerEvent>().add_systems(
            Update,
            handle_connection_on_join_server.run_if(in_state(AppState::Menu)),
        );
    }
}
