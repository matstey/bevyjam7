use crate::movement::TopDownMovementController;
use bevy::prelude::*;

use crate::games::rain::RainAssets;

pub fn duck(assets: &RainAssets) -> impl Bundle {
    let max_speed = 42.0;

    (
        Transform::from_xyz(0.0, -36.5, 20.0),
        Visibility::default(),
        Sprite::from_image(assets.duck.clone()),
        TopDownMovementController {
            max_speed,
            ..default()
        },
    )
}
