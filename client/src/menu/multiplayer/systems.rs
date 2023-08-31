use belly::prelude::*;
use bevy::prelude::*;

use crate::menu::main::systems::set_main_menu_visibility;

use super::components::ServerJoinInput;

// Set
pub fn set_multiplayer_menu_visibility(ctx: &mut EventContext<impl Event>, visible: bool) {
    if visible {
        ctx.select("#multiplayer-menu").remove_class("hidden");
        return;
    }

    ctx.select("#multiplayer-menu").add_class("hidden");
}

pub fn setup_multiplayer_menu(mut commands: Commands) {
    let server_join_input = commands.spawn(ServerJoinInput::default()).id();
    commands.add(eml! {
        <body id="multiplayer-menu" class="hidden">
            <textinput bind:value=to!(server_join_input, ServerJoinInput:server_address) />
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
