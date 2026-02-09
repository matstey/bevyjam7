use bevy::prelude::*;

mod ui_position;
mod ui_rotation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui_rotation::plugin, ui_position::plugin));
}

#[derive(Debug, Clone, Component)]
pub struct Floats;
