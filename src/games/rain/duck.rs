use crate::{
    games::rain::animation::{PlayerAnimation, PlayerAnimationState},
    movement::TopDownMovementController,
};
use bevy::prelude::*;

use crate::audio::sound_effect;
use crate::games::rain::RainAssets;
use rand::prelude::*;

pub fn duck(
    assets: &RainAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    max_speed: f32,
) -> impl Bundle {
    let max_speed = max_speed;

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

pub fn trigger_step_sound_effect(
    mut commands: Commands,
    assets: If<Res<RainAssets>>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::rng();
            let random_step = assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}
