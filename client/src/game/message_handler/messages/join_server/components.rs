use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Component, Serialize, Deserialize, Default)]
pub struct LocalPlayerData {
    pub name: Option<String>,
    pub token: Option<String>,
}
