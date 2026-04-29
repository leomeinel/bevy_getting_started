//! Characters

pub(crate) mod npc;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add plugins
    app.add_plugins(npc::plugin);
}
