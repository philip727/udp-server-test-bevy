use bevy::prelude::*;

use crate::states::AppState;

use self::{
    events::MessageReceivedFromServerEvent,
    messages::join_server::systems::handle_received_sid_message,
    systems::handle_server_to_client_messages,
};

pub mod events;
pub mod messages;
pub mod systems;

pub struct MessageHandlerPlugin;

impl Plugin for MessageHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageReceivedFromServerEvent>()
            .add_systems(
                Update,
                (
                    handle_server_to_client_messages,
                    handle_received_sid_message.run_if(in_state(AppState::Loading)),
                ),
            );
    }
}
