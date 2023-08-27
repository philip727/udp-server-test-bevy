use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{data::ServerDataPlugin, message_handler::ServerMessageHandlerPlugin};

pub mod data;
pub mod message_handler;
pub mod resources;

pub struct ServerPlugins;

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(ServerDataPlugin).add(ServerMessageHandlerPlugin);

        group
    }
}
