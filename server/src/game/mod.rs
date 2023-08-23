use bevy::{prelude::PluginGroup, app::PluginGroupBuilder};

use self::data::ServerDataPlugin;

pub mod connection_manager;
pub mod data;
pub mod resources;

pub struct ServerPlugins;

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(ServerDataPlugin);
        
        group
    }
}
