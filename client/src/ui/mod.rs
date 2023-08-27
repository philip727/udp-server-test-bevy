use bevy::prelude::{Plugin, Update};

use self::interactable::button::systems::handle_button_interaction;

pub mod interactable;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, handle_button_interaction);
    }
}
