//! Theme

pub(crate) mod widgets;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add plugins
    app.add_plugins(widgets::plugin);
}
