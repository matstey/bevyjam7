use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{float::Floats, games::GameData};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn, update));
}

fn spawn(mut commands: Commands, query: Query<(Entity, &UiTransform), Added<Floats>>) {
    for (entity, transform) in query.iter() {
        commands
            .entity(entity)
            .insert((FloatsRotationUiData::new(transform.rotation),));
    }
}

fn update(
    mut query: Query<(&mut UiTransform, &mut FloatsRotationUiData), With<Floats>>,
    data: Res<GameData>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    for (mut transform, mut floats_data) in query.iter_mut() {
        if floats_data.expired(time.elapsed()) {
            // Generate new target position
            // Use the sign of the last target to make sure we always rotate the other way
            floats_data.target = Rot2::degrees(
                rng.random_range(1.0..3.0) * -floats_data.target.as_degrees().signum(),
            );
            floats_data.move_start = transform.rotation;
            floats_data.move_start_time = time.elapsed();
            let speed = rng.random_range(2.0..3.0) * data.round.max(1) as f32; // deg/s
            floats_data.move_duration =
                Duration::from_secs_f32(floats_data.move_distance() / speed);
        } else {
            transform.rotation = floats_data.start * floats_data.slerp(time.elapsed());
        }
    }
}

#[derive(Debug, Clone, Component)]
struct FloatsRotationUiData {
    start: Rot2,
    target: Rot2,
    move_start: Rot2,
    move_start_time: Duration,
    move_duration: Duration,
}

impl FloatsRotationUiData {
    pub fn new(start: Rot2) -> Self {
        Self {
            start,
            target: start,
            move_start: start,
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

    pub fn slerp(&self, time: Duration) -> Rot2 {
        self.move_start
            .slerp(self.target, crate::easing::sine_in_out(self.t(time)))
    }

    pub fn move_distance(&self) -> f32 {
        (self.move_start.angle_to(self.target).to_degrees()).abs()
    }
}
