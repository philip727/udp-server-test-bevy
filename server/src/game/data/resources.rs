use bevy::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct ServerDataManager {
    uuid: Option<Uuid>,
}

impl Default for ServerDataManager {
    fn default() -> Self {
        ServerDataManager { uuid: None }
    }
}

impl ServerDataManager {
    pub fn set_sid(&mut self, uuid: Uuid) {
        self.uuid = Some(uuid);
    }
}
