use std::net::{SocketAddr};

use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    game::connection::events::JoinServerEvent, menu::main::helpers::set_main_menu_visibility,
};

use super::{components::ServerJoinInput, helpers::set_multiplayer_menu_visibility};

pub fn setup_multiplayer_menu(
    mut commands: Commands,
) {
    let server_join_input = commands.spawn(ServerJoinInput::default()).id();
    commands.add(StyleSheet::load("styles/multiplayer-menu.ess"));
    commands.add(eml! {
        <body id="multiplayer-menu" class="hidden">
            <div id="join-server-wrapper">
                <textinput bind:value=to!(server_join_input, ServerJoinInput:server_address) />
                <button on:press=run!(for server_join_input |ctx, s: &ServerJoinInput| {
                    // Sends an event to join the server
                    ctx.send_event(JoinServerEvent(s.server_address.clone()))
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
