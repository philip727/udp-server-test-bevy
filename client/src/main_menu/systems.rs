use bevy::prelude::*;

use crate::ui::interactable::button::{
    builder::ButtonBuilder, helpers::create_button, task::ButtonTask,
};

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
        create_button(
            parent,
            ButtonBuilder {
                text: String::from("Singleplayer"),
                task: ButtonTask::new(|| info!("Singleplayer clicked")),
            },
        );
        create_button(
            parent,
            ButtonBuilder {
                text: String::from("Multiplayer"),
                task: ButtonTask::new(|| info!("Multiplayer clicked")),
            },
        );
        create_button(
            parent,
            ButtonBuilder {
                text: String::from("Settings"),
                task: ButtonTask::new(|| info!("Settings clicked")),
            },
        );
        create_button(
            parent,
            ButtonBuilder {
                text: String::from("Exit"),
                task: ButtonTask::new(|| info!("Exit clicked")),
            },
        );
    });
}
