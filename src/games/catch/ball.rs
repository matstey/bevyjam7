use std::time::Duration;

use bevy::prelude::*;

use crate::{
    games::catch::{CatchAssets, CatchState, balance},
    lifetime::DespawnAfter,
    random::Random2dPosition,
};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Ball {
    pub radius: f32,
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<CatchState>,
    time: Res<Time>,
    assets: Res<CatchAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    random: Res<Random2dPosition>,
) {
    if (time.elapsed() - state.last_release) > state.release_freq {
        state.last_release = time.elapsed();
        if let Some(root) = state.root {
            let ball_entity = commands
                .spawn(ball(
                    &assets,
                    &mut meshes,
                    &mut materials,
                    &random,
                    &time,
                    state.release_freq,
                ))
                .id();
            commands.entity(root).add_child(ball_entity);
        }
    }
}

pub fn ball(
    assets: &CatchAssets,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    random: &Res<Random2dPosition>,
    time: &Res<Time>,
    lifespan: Duration,
) -> impl Bundle {
    (
        Name::new("ball"),
        Ball::new(balance::BALL_RADIUS),
        Mesh2d(meshes.add(Circle::new(balance::BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.5, 0.5, 0.1))),
        Sprite::from_image(assets.ball.clone()),
        Transform::from_translation(random.next(balance::BALL_RADIUS).extend(0.0))
            .with_scale(Vec2::splat(0.25).extend(1.0)),
        DespawnAfter::new(time.elapsed(), lifespan),
    )
}
