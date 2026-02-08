use bevy::prelude::*;

use crate::{
    games::cat_bonk::{CatBonkAssets, GAME},
    screens::Screen,
};

#[derive(Debug, Default, Component)]
pub struct Weapon;

pub fn weapon(assets: &CatBonkAssets) -> impl Bundle {
    (
        Name::new("weapon"),
        Transform::from_xyz(0.0, 0.0, 10.0),
        Visibility::default(),
        DespawnOnExit(GAME),
        DespawnOnExit(Screen::Gameplay),
        Sprite::from_image(assets.weapon.clone()),
        Weapon,
    )
}

pub fn update(
    mut weapon: Single<&mut Transform, With<Weapon>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        weapon.translation.x = cursor_world_pos.x - 80.0;
        weapon.translation.y = cursor_world_pos.y;
    };
}
