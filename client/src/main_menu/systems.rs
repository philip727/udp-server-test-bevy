use bevy::prelude::*;

use crate::ui::interactable::button::helpers::create_button;

pub fn setup_main_menu(mut commands: Commands) {
    let mut root = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(45.),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    });

    root.with_children(|parent| {
        create_button(parent, String::from("Singleplayer"));
        create_button(parent, String::from("Multiplayer"));
        create_button(parent, String::from("Settings"));
        create_button(parent, String::from("Exit"));
    });
}
