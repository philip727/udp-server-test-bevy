use bevy::prelude::*;
use belly::prelude::*;

pub fn set_main_menu_visibility(ctx: &mut EventContext<impl Event>, visible: bool) {
    if visible {
        ctx.select("#main-menu").remove_class("hidden");
        return;
    }

    ctx.select("#main-menu").add_class("hidden");
}
