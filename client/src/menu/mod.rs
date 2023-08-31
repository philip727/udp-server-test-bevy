use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{main::MainMenuPlugin, multiplayer::MultiplayerMenuPlugin};

pub mod main;
pub mod multiplayer;

pub struct MenuUIPlugins;

impl PluginGroup for MenuUIPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(MainMenuPlugin).add(MultiplayerMenuPlugin);

        group
    }
}
