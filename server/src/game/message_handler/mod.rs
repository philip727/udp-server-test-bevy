use bevy::prelude::{Plugin, Update};

use self::{
    events::MessageReceivedFromClientEvent, messages::MessagesPlugin,
    resources::ServerMessageManager, systems::handle_client_to_server_messages,
};

pub mod events;
pub mod helpers;
pub mod messages;
pub mod resources;
pub mod systems;

pub struct ServerMessageHandlerPlugin;

impl Plugin for ServerMessageHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MessagesPlugin)
            .init_resource::<ServerMessageManager>()
            .add_event::<MessageReceivedFromClientEvent>()
            .add_systems(Update, handle_client_to_server_messages);
    }
}
