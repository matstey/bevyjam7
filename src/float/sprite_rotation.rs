use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{float::Floats, games::GameData};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn, update));
}

fn spawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (Added<Floats>, With<Sprite>)>,
) {
    for (entity, transform) in query.iter() {
        commands
            .entity(entity)
            .insert(FloatsRotationSpriteData::new(
                transform.rotation.to_euler(EulerRot::YXZ).2.to_degrees(),
            ));
    }
}

fn update(
    mut query: Query<(&mut Transform, &mut FloatsRotationSpriteData), With<Floats>>,
    data: Res<GameData>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    for (mut transform, mut floats_data) in query.iter_mut() {
        if floats_data.expired(time.elapsed()) {
            // Generate new target position
            floats_data.move_start = floats_data.target;
            // Use the sign of the last target to make sure we always rotate the other way
            floats_data.target = rng.random_range(0.0..2.0) * -floats_data.target.signum();
            floats_data.move_start_time = time.elapsed();
            let speed = rng.random_range(2.0..3.0) * data.fever_grade(); // deg/s
            floats_data.move_duration =
                Duration::from_secs_f32(floats_data.move_distance() / speed);
        } else {
            transform.rotation = Quat::from_axis_angle(
                Vec3::Z,
                (floats_data.start + floats_data.lerp(time.elapsed())).to_radians(),
            );
        }
    }
}

#[derive(Debug, Clone, Component)]
struct FloatsRotationSpriteData {
    start: f32,
    target: f32,
    move_start: f32,
    move_start_time: Duration,
    move_duration: Duration,
}

impl FloatsRotationSpriteData {
    pub fn new(start: f32) -> Self {
        Self {
            start,
            target: 0.0,
            move_start: 0.0,
            move_start_time: Duration::default(),
            move_duration: Duration::default(),
        }
    }

    pub fn expired(&self, time: Duration) -> bool {
        time > self.move_start_time + self.move_duration
    }

    pub fn t(&self, time: Duration) -> f32 {
        ((time.as_secs_f32() - self.move_start_time.as_secs_f32())
            / self.move_duration.as_secs_f32())
        .clamp(0.0, 1.0)
    }

    pub fn lerp(&self, time: Duration) -> f32 {
        self.move_start
            .lerp(self.target, crate::easing::sine_in_out(self.t(time)))
    }

    pub fn move_distance(&self) -> f32 {
        (self.move_start - self.target).abs()
    }
}
