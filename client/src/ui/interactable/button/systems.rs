use bevy::prelude::*;

pub fn handle_button_interaction(
    mut interaction_query: Query<(&Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Pressed");
            },
            Interaction::Hovered => {
                info!("Hovering");
            },
            Interaction::None => {

            },
        }
    }
}
