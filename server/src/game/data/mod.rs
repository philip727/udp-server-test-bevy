pub mod constants;
pub mod errors;
pub mod helpers;
pub mod resources;
pub mod states;
pub mod systems;

use bevy::prelude::*;

use self::{resources::ServerDataManager, systems::load_sid};

pub struct ServerDataPlugin;

impl Plugin for ServerDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerDataManager>()
            .add_systems(Startup, load_sid);
    }
}
