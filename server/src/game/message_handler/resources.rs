use bevy::prelude::*;

#[derive(Resource)]
pub struct ServerMessageManager {}

impl Default for ServerMessageManager {
    fn default() -> Self {
        ServerMessageManager {}
    }
}
