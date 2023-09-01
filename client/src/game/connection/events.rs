use bevy::prelude::Event;

#[derive(Event)]
pub struct JoinServerEvent(pub String);
