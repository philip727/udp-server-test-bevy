use bevy::prelude::*;

use super::builder::ButtonBuilder;

pub fn create_button(parent: &mut ChildBuilder<'_, '_, '_>, builder: ButtonBuilder) -> Entity {
    let mut button = parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(290.),
                height: Val::Px(50.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgb_u8(255, 255, 255)),
            ..Default::default()
        },
        builder.task,
    ));

    button.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            builder.text,
            TextStyle {
                font_size: 40.,
                color: Color::rgb_u8(0, 0, 0),
                ..Default::default()
            },
        ));
    });

    button.id()
}
