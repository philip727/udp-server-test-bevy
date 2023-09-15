use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{connection::HandleConnectionPlugin, message_handler::MessageHandlerPlugin};

pub mod camera;
pub mod connection;
pub mod message_handler;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(HandleConnectionPlugin).add(MessageHandlerPlugin);

        group
    }
}
