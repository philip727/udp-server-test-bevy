use belly::prelude::*;
use bevy::prelude::*;

pub fn set_multiplayer_menu_visibility(ctx: &mut EventContext<impl Event>, visible: bool) {
    if visible {
        ctx.select("#multiplayer-menu").remove_class("hidden");
        return;
    }

    ctx.select("#multiplayer-menu").add_class("hidden");
}
