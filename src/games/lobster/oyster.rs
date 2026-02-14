use std::time::Duration;

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::audio::sound_effect;
use crate::lifetime::DespawnAfter;
use bevy::prelude::*;
use rand::Rng;

use crate::games::lobster::{LobsterAssets, LobsterState, balance};

#[derive(Debug, Default, Component)]
pub struct Oyster {
    is_open: bool,
}

#[derive(Component)]
pub struct PlaySoundDelayed(Timer, Handle<AudioSource>);

#[derive(Debug, Default, Component)]
pub struct Pearl;

#[derive(Component, Deref, DerefMut)]
pub struct OpenTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CloseTimer(Timer);

pub fn oyster(
    assets: &LobsterAssets,
    pos: Vec2,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    open_time: f32,
) -> impl Bundle {
    // use bevy random source?
    let mut rng = rand::rng();
    let delay = rng.random_range(balance::MIN_OPEN_DELAY..balance::MAX_OPEN_DELAY);

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 220, y: 253 }, 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut close_timer = Timer::from_seconds(open_time, TimerMode::Once);
    close_timer.pause();

    (
        Name::new("oyster"),
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
        children![pearl(assets)],
    )
}

fn pearl(assets: &LobsterAssets) -> impl Bundle {
    (
        Name::new("pearl"),
        Transform::from_xyz(-16.0, -16.0, 3.0),
        Visibility::Hidden,
        Sprite::from_image(assets.pearl.clone()),
        Pearl,
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
    pearl: Single<&mut Visibility, With<Pearl>>,
) {
    let (mut oyster, mut open_timer, mut close_timer, mut sprite, mut anim_timer) =
        query.into_inner();
    open_timer.tick(time.delta());
    close_timer.tick(time.delta());

    let mut vis = pearl.into_inner();

    if open_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        atlas.index = 2;
        anim_timer.pause();
        close_timer.unpause();
        oyster.is_open = true;
        *vis = Visibility::Visible;
    }

    if close_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        atlas.index = 0;
        oyster.is_open = false;
        *vis = Visibility::Hidden;
    }
}

pub fn try_grab(
    oyster: Single<(Entity, &Oyster, &mut CloseTimer, &mut OpenTimer)>,
    pearl: Single<(Entity, &Pearl)>,
    mut state: ResMut<LobsterState>,
    mut commands: Commands,
    assets: Res<LobsterAssets>,
    time: Res<Time>,
) {
    let (entity, oyster, mut close_timer, mut open_timer) = oyster.into_inner();
    close_timer.pause();
    open_timer.pause();
    state.caught = Some(oyster.is_open);

    let sfx = if oyster.is_open {
        assets.pearl_hit_sfx.clone()
    } else {
        assets.pearl_miss_sfx.clone()
    };
    commands.entity(entity).insert(PlaySoundDelayed(
        Timer::from_seconds(0.4, TimerMode::Once),
        sfx,
    ));

    let (pearl_entity, _) = pearl.into_inner();
    commands.entity(pearl_entity).insert(DespawnAfter::new(
        time.elapsed(),
        Duration::from_secs_f32(0.4),
    ));
}

pub fn play_sound_after_delay(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut PlaySoundDelayed>,
) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.spawn(sound_effect(timer.1.clone()));
        }
    }
}
