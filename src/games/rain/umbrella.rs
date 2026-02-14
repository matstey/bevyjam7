use bevy::prelude::*;
use rand::Rng;

use crate::games::{camera::shake::CameraShakeState, rain::RainAssets};

#[derive(Debug, Default, Component)]
pub struct RandomMover {
    velocity: f32,
    end_time: f32,
}

pub fn umbrella(assets: &RainAssets) -> impl Bundle {
    (
        Transform::from_xyz(0.0, -0.0, 1.0),
        Visibility::default(),
        Sprite::from_image(assets.umbrella.clone()),
        RandomMover {
            velocity: 0.0,
            end_time: 0.0,
        },
        children![(
            Transform::from_xyz(0.0, -120.0, 1.0),
            Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(43.0, 200.0)),
                ..default()
            }
        )],
    )
}

pub fn update(time: Res<Time>, mut query: Query<(&mut RandomMover, &mut Transform)>) {
    let mut rng = rand::rng();
    let bound = Vec2 { x: 70.0, y: 32.0 };

    for (mut mover, mut transform) in &mut query {
        if time.elapsed_secs() > mover.end_time {
            mover.end_time = time.elapsed_secs() + rng.random_range(0.5..1.5);
            mover.velocity = rng.random_range(-30.0..30.0);
            info!("new velocity: {}", mover.velocity);
        }

        if f32::abs(transform.translation.x) > bound.x {
            mover.end_time = 0.0;
            mover.velocity = 0.0;
            info!("stuck, resetting velocity");
        }

        transform.translation.x += mover.velocity * time.delta_secs();
        transform.translation.x = f32::floor(transform.translation.x);

        info!("x: {}", transform.translation.x);
    }
}
