/*
 * Heavily inspired by:
 * - https://github.com/ickshonpe/bevy_ui_text_input/blob/master/examples/multiple_inputs.rs
 */

//! Text input widget

use bevy::{color::palettes::tailwind, input_focus::InputFocus, prelude::*};
use bevy_ui_text_input::SubmitText;

pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<InputError>();
    app.add_message::<InputSubmitted>();
    app.add_message::<InputUsed>();

    // Add update systems
    app.add_systems(Update, (on_submit, focus, on_error, on_used));
}

pub(crate) const OUTLINE_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;
pub(crate) const OUTLINE_COLOR_ERROR: Srgba = tailwind::RED_500;
pub(crate) const OUTLINE_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;

/// Message for unsuccessful input submission
#[derive(Message)]
pub(crate) struct InputError(Entity);

/// Message for input submission
#[derive(Message)]
pub(crate) struct InputSubmitted {
    /// Entity of input
    pub(crate) entity: Entity,
    /// Text from input submission
    pub(crate) text: String,
}

/// Message for validated input submission
#[derive(Message)]
pub(crate) struct InputUsed(Entity);

/// Validate input on `is_error` condition
///
/// On error, this writes [`InputError`] and on success, it writes [`InputUsed`].
pub(crate) fn validate_input(
    error_msg: &mut MessageWriter<InputError>,
    used_msg: &mut MessageWriter<InputUsed>,
    entity: Entity,
    is_error: bool,
) -> bool {
    // Write InputError and return false if we have an error
    if is_error {
        error_msg.write(InputError(entity));
        return false;
    }

    // Write InputUsed and return true
    used_msg.write(InputUsed(entity));
    true
}

/// Read messages of type [`SubmitText`]
fn on_submit(
    mut msgs: MessageReader<SubmitText>,
    mut error_msg: MessageWriter<InputError>,
    mut submitted_msg: MessageWriter<InputSubmitted>,
) {
    for msg in msgs.read() {
        let entity = msg.entity;
        let text = msg.text.trim();

        // Continue if text is empty and write InputError
        if text.is_empty() {
            error_msg.write(InputError(entity));
            continue;
        }

        // Write InputSubmitted
        submitted_msg.write(InputSubmitted {
            entity,
            text: text.to_string(),
        });
    }
}

/// Update outline color based on focus
fn focus(query: Query<(Entity, &mut Outline)>, input_focus: Res<InputFocus>) {
    // Return if input focus has not changed
    if !input_focus.is_changed() {
        return;
    }

    // Change outline color based on focus
    for (entity, mut outline) in query {
        if input_focus.0.is_some_and(|active| active == entity) {
            outline.color = OUTLINE_COLOR_ACTIVE.into();
        } else {
            outline.color = OUTLINE_COLOR_INACTIVE.into();
        }
    }
}

/// Update outline color on input error
fn on_error(mut msgs: MessageReader<InputError>, mut query: Query<(Entity, &mut Outline)>) {
    for msg in msgs.read() {
        // Find outline matching entity from outline query and set color
        if let Some((_e, mut outline)) = query.iter_mut().find(|(e, _name)| *e == msg.0) {
            outline.color = OUTLINE_COLOR_ERROR.into();
        }
    }
}

/// Update outline color on used input
fn on_used(mut msgs: MessageReader<InputUsed>, mut query: Query<(Entity, &mut Outline)>) {
    for msg in msgs.read() {
        // Find outline matching entity from outline query and set color
        if let Some((_e, mut outline)) = query.iter_mut().find(|(e, _name)| *e == msg.0) {
            outline.color = OUTLINE_COLOR_ACTIVE.into();
        }
    }
}
