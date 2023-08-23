use bevy::prelude::*;

use super::data::ServerData;

#[derive(Resource)]
pub struct ServerManager {
    data: ServerData
}

impl Default for ServerManager {
    fn default() -> Self {
        ServerManager {
            data: ServerData::new(),
        }
    }
}
