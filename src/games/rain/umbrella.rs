use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;

use crate::games::rain::RainAssets;

#[derive(Debug, Default, Component)]
pub struct RandomMover {
    velocity: f32,
    end_time: f32,
}

#[derive(Component)]
pub struct Umbrella;

pub fn umbrella(assets: &RainAssets) -> impl Bundle {
    (
        Umbrella,
        Transform::from_xyz(0.0, -0.5, 10.0),
        Visibility::default(),
        Sprite::from_image(assets.umbrella.clone()),
        RandomMover {
            velocity: 0.0,
            end_time: 0.0,
        },
        children![(
            Transform::from_xyz(0.0, 4.0, -1.0),
            Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(43.0, 48.0)),
                ..default()
            },
            Anchor::TOP_CENTER
        )],
    )
}

pub fn update(time: Res<Time>, mut query: Query<(&mut RandomMover, &mut Transform)>) {
    let mut rng = rand::rng();
    let bound = Vec2 { x: 70.0, y: 32.0 };

    for (mut mover, mut transform) in &mut query {
        if time.elapsed_secs() > mover.end_time {
            mover.end_time = time.elapsed_secs() + rng.random_range(0.5..1.5);
            mover.velocity =
                rng.random_range(15.0..30.0) * f32::signum(rng.random_range(-1.0..1.0));
        }

        let new_x = transform.translation.x + (mover.velocity * time.delta_secs());
        if f32::abs(new_x) > bound.x {
            mover.end_time = 0.0;
            mover.velocity = 0.0;
        } else {
            transform.translation.x = new_x;
        }
    }
}
