use bevy::prelude::*;

use crate::games::cat_bonk::{CatBonkAssets, CatBonkState};

#[derive(Debug, Default, Component)]
pub struct Cat;

pub fn cat(assets: &CatBonkAssets, pos: Vec2) -> impl Bundle {
    (
        Name::new("cat"),
        Transform::from_xyz(pos.x, pos.y, 1.0),
        Visibility::default(),
        Sprite::from_image(assets.cat.clone()),
        Pickable::default(),
        Cat,
    )
}

// todo: pop up after random time period
pub fn update() {}

pub fn on_hit(click: On<Pointer<Click>>, mut commands: Commands, mut state: ResMut<CatBonkState>) {
    commands.entity(click.entity).despawn();
    state.hit_count += 1;
}
