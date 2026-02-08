use crate::games::cat_bonk::CatBonkAssets;
use bevy::{camera::visibility::RenderLayers, prelude::*};

#[derive(Debug, Default, Component)]
pub struct Background;

pub fn level(assets: &CatBonkAssets) -> impl Bundle {
    (
        Name::new("background"),
        Transform::from_scale(Vec3::splat(1.0)),
        Visibility::default(),
        Sprite::from_image(assets.background.clone()),
        RenderLayers::default(),
        Background,
    )
}

// todo: replace this with a timed move component or something
pub fn update(time: Res<Time>, mut background: Single<&mut Transform, With<Background>>) {
    background.translation.x = f32::sin(time.elapsed_secs() * 5.0) * 10.0 - 200.0;
}
