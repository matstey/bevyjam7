use bevy::prelude::*;
use std::time::Duration;

use crate::audio::sound_effect;
use crate::games::lobster::LobsterAssets;

#[derive(Debug, Default, Component)]
pub struct MoveToTarget {
    target: Vec2,
    time: f32,
    start_time: Option<Duration>,
    start_location: Vec2,
}

#[derive(Debug, Default, Component)]
pub struct Lobster;

pub fn lobster_char(assets: &LobsterAssets, pos: Vec2, target: Vec2) -> impl Bundle {
    (
        Name::new("lobster"),
        Transform::from_xyz(pos.x, pos.y, 10.0).with_scale(Vec3 {
            x: 0.8,
            y: 0.8,
            z: 1.0,
        }),
        Visibility::default(),
        Sprite::from_image(assets.lobster.clone()),
        MoveToTarget {
            target,
            time: 0.5,
            start_time: None,
            start_location: pos,
        },
        Lobster,
    )
}

// todo: share this?
pub fn update_move(time: Res<Time>, moveable_query: Query<(&mut Transform, &MoveToTarget)>) {
    for (mut transform, target) in moveable_query {
        // dumb lerp towards target
        if let Some(start) = target.start_time {
            let t = (time.elapsed() - start).div_f32(target.time);
            let p = target
                .start_location
                .lerp(target.target, crate::easing::cubic_in_out(t.as_secs_f32()));
            transform.translation.x = p.x;
            transform.translation.y = p.y;
        }
    }
}

pub fn try_grab(
    mut commands: Commands,
    assets: Res<LobsterAssets>,
    time: Res<Time>,
    mut lobster: Single<&mut MoveToTarget, With<Lobster>>,
) {
    lobster.start_time = Some(time.elapsed());
    commands.spawn(sound_effect(assets.lobster_go.clone()));
}
