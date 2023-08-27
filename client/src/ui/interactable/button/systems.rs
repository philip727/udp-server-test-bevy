use bevy::prelude::*;

use super::task::ButtonTask;

pub fn handle_button_interaction(
    mut interaction_query: Query<(&Interaction, &ButtonTask), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_task) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Pressed");
                button_task.execute();
            },
            Interaction::Hovered => {
                info!("Hovering");
            },
            Interaction::None => {

            },
        }
    }
}
