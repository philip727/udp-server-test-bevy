use bevy::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct ServerDataManager {
    server_id: Option<Uuid>,
}

impl Default for ServerDataManager {
    fn default() -> Self {
        ServerDataManager { server_id: None }
    }
}

impl ServerDataManager {
    pub fn set_server_id(&mut self, uuid: Uuid) {
        self.server_id = Some(uuid);
    }

    pub fn get_server_id(&self) -> Option<&Uuid> {
        self.server_id.as_ref()
    }
}
