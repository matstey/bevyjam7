use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
};

pub mod shake;

pub const RENDERLAYER_GAME: RenderLayers = RenderLayers::layer(1);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(shake::plugin);
}
