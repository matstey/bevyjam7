use crate::{
    animation::{AnimationIndices, AnimationTimer},
};
use bevy::prelude::*;
use rand::Rng;

use crate::games::lobster::{LobsterAssets, LobsterState, balance};

#[derive(Debug, Default, Component)]
pub struct Oyster
{
    is_open: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct OpenTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CloseTimer(Timer);

pub fn oyster(
    assets: &LobsterAssets,
    pos: Vec2,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // use bevy random source?
    let mut rng = rand::rng();
    let delay = rng.random_range(balance::MIN_OPEN_DELAY..balance::MAX_OPEN_DELAY);

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 220, y: 253 }, 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Can I make one that starts paused?
    let mut close_timer = Timer::from_seconds(balance::OPEN_TIME, TimerMode::Once);
    close_timer.pause();

    (
        Name::new("cat"),
        Transform::from_xyz(pos.x, pos.y, 2.0),
        Visibility::default(),
        Sprite::from_atlas_image(
            assets.oyster.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        AnimationIndices { first: 0, last: 1 },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        Pickable::default(),
        Oyster::default(),
        OpenTimer(Timer::from_seconds(delay, TimerMode::Once)),
        CloseTimer(close_timer),
    )
}

pub fn update(
    time: Res<Time>,
    query: Single<(
        &mut Oyster,
        &mut OpenTimer,
        &mut CloseTimer,
        &mut Sprite,
        &mut AnimationTimer,
    )>,
) {
    let (mut oyster, mut open_timer, mut close_timer, mut sprite, mut anim_timer) = query.into_inner();
    open_timer.tick(time.delta());
    close_timer.tick(time.delta());

    if open_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        atlas.index = 2;
        anim_timer.pause();
        close_timer.unpause();
        oyster.is_open = true;
    }

    if close_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        atlas.index = 0;
        oyster.is_open = false;
    }
}

pub fn try_grab(oyster: Single<(&Oyster, &mut CloseTimer, &mut OpenTimer)>, mut state: ResMut<LobsterState>,) {

    let (oyster, mut close_timer, mut open_timer) = oyster.into_inner();
    close_timer.pause();
    open_timer.pause();
    state.caught = Some(oyster.is_open);
}
