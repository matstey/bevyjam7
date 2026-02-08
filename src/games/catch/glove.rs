use bevy::prelude::*;

use crate::{
    games::catch::CatchAssets,
    movement::{ScreenWrap, TopDownMovementController},
};

pub fn glove(max_speed: f32, assets: &CatchAssets) -> impl Bundle {
    (
        Name::new("Glove"),
        Sprite::from_image(assets.glove.clone()),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        TopDownMovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
    )
}
