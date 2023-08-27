use bevy::prelude::info;

use crate::game::data::helpers::{check_for_sid, create_sid};

use super::resources::ServerDataManager;
use bevy::prelude::*;

pub fn load_sid(mut server_data_manager: ResMut<ServerDataManager>) {
    let mut has_sid = check_for_sid();

    // If it cant find a sid then we need to create one
    if let None = has_sid {
        has_sid =
            Some(create_sid().expect("Failed to create sid, please try running again."));
    }

    info!("Found server id {}.", has_sid.unwrap());
    server_data_manager.set_server_id(has_sid.unwrap());
}
