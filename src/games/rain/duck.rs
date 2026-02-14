use crate::{games::rain::animation::PlayerAnimation, movement::TopDownMovementController};
use bevy::prelude::*;

use crate::games::rain::RainAssets;

pub fn duck(
    assets: &RainAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let max_speed = 42.0;

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 32, y: 32 }, 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_anim = PlayerAnimation::new();

    (
        Transform::from_xyz(0.0, -36.5, 20.0),
        Visibility::default(),
        Sprite::from_atlas_image(
            assets.duck.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_anim.get_atlas_index(),
            },
        ),
        TopDownMovementController {
            max_speed: Vec2 {
                x: max_speed,
                y: 0.0,
            },
            ..default()
        },
        player_anim,
    )
}
