use belly::prelude::*;
use bevy::prelude::*;


use crate::menu::main::helpers::set_main_menu_visibility;

use super::{components::ServerJoinInput, helpers::set_multiplayer_menu_visibility};

pub fn setup_multiplayer_menu(mut commands: Commands) {
    let server_join_input = commands.spawn(ServerJoinInput::default()).id();

    commands.add(StyleSheet::load("styles/multiplayer-menu.ess"));
    commands.add(eml! {
        <body id="multiplayer-menu" class="hidden">
            <div id="join-server-wrapper">
                <textinput bind:value=to!(server_join_input, ServerJoinInput:server_address) />
                <button on:press=run!(for server_join_input |ctx, s: &ServerJoinInput| {
                    info!("server: {:?}", s.server_address);
                })>
                    "Join"
                </button>
            </div>
            <button on:press=run!(|ctx| {
                set_main_menu_visibility(ctx, true);
                set_multiplayer_menu_visibility(ctx, false);
            })>
                "Back"
            </button>
        </body>
    });
}

pub fn get_server_address(
    server_join_input_query: Query<&ServerJoinInput, Changed<ServerJoinInput>>,
) {
    let server_join_input = server_join_input_query.get_single();

    if let Ok(server_join_input) = server_join_input {
        info!("Server addy: {}", server_join_input.server_address);
    }
}
