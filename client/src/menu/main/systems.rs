use belly::prelude::*;
use bevy::prelude::*;

use crate::menu::multiplayer::systems::set_multiplayer_menu_visibility;

pub fn set_main_menu_visibility(ctx: &mut EventContext<impl Event>, visible: bool) {
    if visible {
        ctx.select("#main-menu").remove_class("hidden");
        return;
    }

    ctx.select("#main-menu").add_class("hidden");
}

pub fn setup_main_menu(mut commands: Commands) {
    let body = commands.spawn_empty().id();

    commands.add(StyleSheet::load("styles/main-menu.ess"));
    commands.add(eml! {
        <body {body} id="main-menu">
            <button>
                "Singleplayer"
            </button>
            <button on:press=run!(|ctx| {
                set_main_menu_visibility(ctx, false);
                set_multiplayer_menu_visibility(ctx, true);
            })>
                "Multiplayer"
            </button>
            <button>
                "Settings"
            </button>
            <button>
                "Exit"
            </button>
        </body>
    });
}
