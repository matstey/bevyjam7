use bevy::prelude::*;

pub mod progress_bar;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(progress_bar::plugin);
}
