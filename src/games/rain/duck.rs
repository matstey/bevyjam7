use crate::movement::TopDownMovementController;
use bevy::{prelude::*, sprite::Anchor};

use crate::games::rain::RainAssets;

pub fn duck(assets: &RainAssets) -> impl Bundle {
    let max_speed = 42.0;

    (
        Transform::from_xyz(0.0, -40.5, 20.0),
        Visibility::default(),
        Sprite::from_image(assets.duck.clone()),
        TopDownMovementController {
            max_speed: Vec2 {
                x: max_speed,
                y: 0.0,
            },
            ..default()
        },
    )
}
