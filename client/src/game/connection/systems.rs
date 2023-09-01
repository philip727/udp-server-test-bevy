use bevy::prelude::*;

use super::events::JoinServerEvent;

pub fn handle_join_server_events(mut join_server_event_reader: EventReader<JoinServerEvent>) {
    for event in &mut join_server_event_reader {
        info!("joining server: {:?}", event.0);
    }
}
