use bevy::{prelude::*, app::AppExit};

use crate::ui::interactable::button::{builder::ButtonBuilder, helpers::create_button};

use super::components::{ExitButton, MultiplayerButton, SettingsButton, SingleplayerButton};

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
            SingleplayerButton,
            ButtonBuilder {
                text: String::from("Singleplayer"),
            },
        );

        create_button(
            parent,
            MultiplayerButton,
            ButtonBuilder {
                text: String::from("Multiplayer"),
            },
        );

        create_button(
            parent,
            SettingsButton,
            ButtonBuilder {
                text: String::from("Settings"),
            },
        );

        create_button(
            parent,
            ExitButton,
            ButtonBuilder {
                text: String::from("Exit"),
            },
        );
    });
}

pub fn handle_singleplayer_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SingleplayerButton>, With<Button>),
    >,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Singleplayer button");
            }
            Interaction::Hovered => {
                info!("Hovering");
            }
            Interaction::None => {}
        }
    }
}

pub fn handle_multiplayer_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<MultiplayerButton>, With<Button>),
    >,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Multiplayer button");
            }
            Interaction::Hovered => {
                info!("Hovering");
            }
            Interaction::None => {}
        }
    }
}

pub fn handle_settings_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SettingsButton>, With<Button>),
    >,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Settings button");
            }
            Interaction::Hovered => {
                info!("Hovering");
            }
            Interaction::None => {}
        }
    }
}

pub fn handle_exit_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<ExitButton>, With<Button>),
    >,
    mut exit_event_writer: EventWriter<AppExit>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {

            }
            Interaction::None => {}
        }
    }
}
