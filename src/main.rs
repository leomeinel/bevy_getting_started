/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Main with [`AppPlugin`]

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod characters;
mod theme;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ui_text_input::TextInputPlugin;

/// Main function
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

/// AppPlugin that adds everything this app needs to run
struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "bevy_getting_started".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            TextInputPlugin,
        ));

        // Add other plugins.
        app.add_plugins((theme::plugin, characters::plugin));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// Spawn [`Camera2d`]
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
