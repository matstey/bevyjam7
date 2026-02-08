use bevy::prelude::*;

use crate::{
    games::cat_bonk::CatBonkAssets,
};

#[derive(Debug, Default, Component)]
pub struct Weapon;

pub fn weapon(assets: &CatBonkAssets) -> impl Bundle {
    (
        Name::new("weapon"),
        Transform::from_xyz(0.0, 0.0, 2.0),
        Visibility::default(),
        Sprite {
            image: assets.weapon.clone(),
            custom_size: Some(Vec2::new(160.0, 174.0)),
            ..default()
        },
        Weapon,
    )
}

pub fn update(
    mut weapon: Single<&mut Transform, With<Weapon>>,
    camera_query: Single<(&Camera, &GlobalTransform), Without<IsDefaultUiCamera>>,
    window: Single<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let sprite_offset = Vec2 { x: -60.0, y: 50.0 };
        weapon.translation.x = cursor_world_pos.x + sprite_offset.x;
        weapon.translation.y = cursor_world_pos.y + sprite_offset.y;
    };

    let angle: f32 = if buttons.pressed(MouseButton::Left) {
        -60.0
    } else {
        0.0
    };

    weapon.rotation = Quat::from_rotation_z(angle.to_radians());
}
