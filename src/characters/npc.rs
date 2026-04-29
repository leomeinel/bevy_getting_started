/*
 * Heavily inspired by:
 * - https://bevy.org/learn/quick-start/getting-started
 */

//! Npc

use bevy::{color::palettes::tailwind, platform::collections::HashMap, prelude::*};
use bevy_ui_text_input::{TextInputFilter, TextInputMode, TextInputNode, TextInputPrompt};

use crate::theme::widgets::{
    grid::{self, GridMarker0, GridMarker1},
    text_input::{self, InputError, InputSubmitted, InputUsed},
};

pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<Renamed>();

    // Insert resources
    app.init_resource::<InputMap>();
    app.init_resource::<OutputMap>();

    // Add startup systems
    app.add_systems(Startup, setup.after(grid::setup));

    // Add update systems
    app.add_systems(
        Update,
        (spawn_rename, create_on_input, rename_on_input, on_renamed),
    );
}

/// Message that gets written on successful renaming
#[derive(Message)]
struct Renamed {
    /// [`Entity`] of the [`Npc`]
    npc_entity: Entity,
    /// [`Entity`] of name output
    name_output_entity: Entity,
    /// Text from input submission
    text: String,
}

/// Npc
#[derive(Component)]
struct Npc;

/// Name
#[derive(Component)]
struct Name(String);

/// Name input map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name input [`Entity`]'
#[derive(Resource, Deref, Default)]
struct InputMap(HashMap<Entity, Entity>);

/// Name output map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name [`Entity`]'
#[derive(Resource, Deref, Default)]
struct OutputMap(HashMap<Entity, Entity>);

/// Marker component for any name input that is used for renaming
#[derive(Component)]
struct RenameMarker;

/// Spawn name input for creating a new [`Npc`]
fn setup(
    grid: Single<Entity, With<GridMarker0>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let grid_entity = grid.entity();

    // Spawn as child of grid_entity
    commands.entity(grid_entity).with_children(|commands| {
        commands.spawn(Text::new("Enter name"));
        commands
            .spawn(input(&assets, "Create Npc"))
            .insert(input_filter());
    });
}

/// Create the name inputs for renaming every [`Npc`]
fn spawn_rename(
    grid: Single<Entity, With<GridMarker1>>,
    npc_query: Query<(Entity, &Name), With<Npc>>,
    mut commands: Commands,
    mut input_map: ResMut<InputMap>,
    mut output_map: ResMut<OutputMap>,
    assets: Res<AssetServer>,
) {
    // Get npc entity and npc name from npc query
    // This gets any npc that does not have an associated npc in the input map
    let Some((npc_entity, name)) = npc_query
        .iter()
        .find(|(e, _name)| !input_map.0.contains_key(e))
    else {
        return;
    };

    let grid_entity = grid.entity();

    // Spawn as child of grid_entity
    commands.entity(grid_entity).with_children(|commands| {
        // Spawn name input and name output
        let name_input_entity = commands
            .spawn((input(&assets, "Rename Npc"), RenameMarker))
            .insert(input_filter())
            .id();
        let name_output_entity = commands.spawn(Text::new(name.0.as_str())).id();

        // Insert into maps
        input_map.0.insert(npc_entity, name_input_entity);
        output_map.0.insert(npc_entity, name_output_entity);
    });
}

/// Add [`Npc`]s from [`InputSubmitted`]
///
/// This adds a bundle of [`Npc`] and [`Name`] from [`InputSubmitted`]
fn create_on_input(
    mut msgs: MessageReader<InputSubmitted>,
    mut error_msg: MessageWriter<InputError>,
    mut used_msg: MessageWriter<InputUsed>,
    npc_query: Query<&Name, With<Npc>>,
    renamed_query: Query<Entity, With<RenameMarker>>,
    mut commands: Commands,
) {
    for msg in msgs.read() {
        // Continue if targeting a renamed entity
        let entity = msg.entity;
        if renamed_query.contains(entity) {
            continue;
        }

        let name = msg.text.clone();

        // Continue if an Npc with the same name already exists
        if !text_input::validate_input(
            &mut error_msg,
            &mut used_msg,
            entity,
            npc_query.iter().any(|npc_name| npc_name.0 == name),
        ) {
            continue;
        }

        commands.spawn((Npc, Name(name)));
    }
}

/// Rename [`Npc`]s from [`InputSubmitted`]
fn rename_on_input(
    mut msgs: MessageReader<InputSubmitted>,
    mut error_msg: MessageWriter<InputError>,
    mut renamed_msg: MessageWriter<Renamed>,
    mut used_msg: MessageWriter<InputUsed>,
    mut npc_query: Query<(Entity, &mut Name), With<Npc>>,
    renamed_query: Query<Entity, With<RenameMarker>>,
    input_map: Res<InputMap>,
    output_map: Res<OutputMap>,
) {
    for msg in msgs.read() {
        // Continue if not targeting a renamed entity
        let entity = msg.entity;
        if !renamed_query.contains(entity) {
            continue;
        }

        let name = msg.text.clone();

        // Continue if an Npc with the same name already exists
        if !text_input::validate_input(
            &mut error_msg,
            &mut used_msg,
            entity,
            npc_query.iter().any(|(_e, npc_name)| npc_name.0 == name),
        ) {
            continue;
        }

        // Get npc entity and npc name from npc query
        // Find npc matching entity and check that it exists in input map
        let Some((npc_entity, mut npc_name)) = npc_query
            .iter_mut()
            .find(|(e, _name)| input_map.get(e).is_some_and(|e| *e == entity))
        else {
            continue;
        };

        // Get name output entity from output map
        let Some(name_output_entity) = output_map.get(&npc_entity) else {
            continue;
        };

        // Mutate npc name and write Renamed message
        npc_name.0 = name.clone();
        renamed_msg.write(Renamed {
            npc_entity,
            name_output_entity: *name_output_entity,
            text: name.clone(),
        });
    }
}

/// Modify text of name output on renamed
fn on_renamed(
    mut msgs: MessageReader<Renamed>,
    npc_query: Query<Entity, With<Npc>>,
    mut text_query: Query<&mut Text>,
) {
    for msg in msgs.read() {
        // Continue if not targeting the msg npc entity
        if !npc_query.contains(msg.npc_entity) {
            continue;
        }

        // Modify text of name output entity via text query
        if let Ok(mut text) = text_query.get_mut(msg.name_output_entity) {
            text.0 = msg.text.clone();
        }
    }
}

/// [`Bundle`] containing name input
fn input(assets: &Res<AssetServer>, prompt: &str) -> impl Bundle {
    (
        TextInputNode {
            mode: TextInputMode::SingleLine,
            max_chars: Some(20),
            ..default()
        },
        TextFont {
            font: assets.load("fonts/Fira_Mono/FiraMono-Medium.ttf"),
            font_size: 20.,
            ..default()
        },
        TextInputPrompt::new(prompt),
        TextColor(tailwind::NEUTRAL_100.into()),
        Node {
            width: Val::Px(200.0),
            height: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(tailwind::NEUTRAL_800.into()),
        Outline {
            width: Val::Px(2.0),
            offset: Val::Px(2.0),
            color: text_input::OUTLINE_COLOR_INACTIVE.into(),
        },
    )
}

/// Name input filter
///
/// This filters for anything that is alphanumeric or whitespace.
fn input_filter() -> TextInputFilter {
    TextInputFilter::custom(is_alphanumeric_or_whitespace)
}

/// Check if text is alphanumeric or whitespace
fn is_alphanumeric_or_whitespace(text: &str) -> bool {
    text.chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
}
