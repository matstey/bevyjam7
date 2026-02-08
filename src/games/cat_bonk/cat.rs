use bevy::prelude::*;

use crate::games::cat_bonk::{CatBonkAssets};

#[derive(Debug, Default, Component)]
pub struct Cat;

pub fn cat(assets: &CatBonkAssets, pos: Vec2) -> impl Bundle {
    (
        Name::new("cat"),
        Transform::from_xyz(pos.x, pos.y, 1.0),
        Visibility::default(),
        Sprite::from_image(
            assets.cat.clone(),
        ),
        Cat
    )
}

// todo: pop up after random time period
pub fn update() {
}
