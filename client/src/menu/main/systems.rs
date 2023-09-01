use belly::prelude::*;
use bevy::prelude::*;

use crate::menu::multiplayer::helpers::set_multiplayer_menu_visibility;

use super::helpers::set_main_menu_visibility;


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
