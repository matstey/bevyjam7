use bevy::prelude::*;

use crate::{
    games::catch::{CatchAssets, balance},
    movement::{ScreenWrap, TopDownMovementController},
};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Glove {
    pub radius: f32,
}

impl Glove {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn glove(
    max_speed: f32,
    assets: &CatchAssets,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        Name::new("glove"),
        Glove::new(balance::GLOVE_RADIUS),
        Mesh2d(meshes.add(Circle::new(balance::GLOVE_RADIUS))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.5, 0.3))),
        Sprite::from_image(assets.glove.clone()),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)), // TODO: Random start position??
        TopDownMovementController {
            max_speed: Vec2::splat(max_speed),
            ..default()
        },
        ScreenWrap,
    )
}
