//! Widgets

pub(crate) mod grid;
pub(crate) mod text_input;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    // Add plugins
    app.add_plugins((grid::plugin, text_input::plugin));
}
