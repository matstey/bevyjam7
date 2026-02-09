use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{float::Floats, games::GameData, random};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn, update));
}

fn spawn(
    mut commands: Commands,
    query: Query<(Entity, &UiTransform), Added<Floats>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (entity, transform) in query.iter() {
        commands
            .entity(entity)
            .insert((FloatsPositionUiData::new(transform.translation.resolve(
                window.scale_factor(),
                window.size(),
                window.size(),
            )),));
    }
}

fn update(
    mut query: Query<(&mut UiTransform, &mut FloatsPositionUiData), With<Floats>>,
    data: Res<GameData>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    for (mut transform, mut float_data) in query.iter_mut() {
        if float_data.expired(time.elapsed()) {
            // Generate new target position
            float_data.move_start = float_data.target; // Assume we met target. Saves using `resolve()`
            float_data.target = Vec2::new(
                rng.random_range(2.0..10.0) * random::sign(&mut rng),
                rng.random_range(2.0..10.0) * random::sign(&mut rng),
            );
            float_data.move_start_time = time.elapsed();
            let speed = rng.random_range(5.0..10.0) * data.round.max(1) as f32; // px/s
            float_data.move_duration = Duration::from_secs_f32(float_data.move_distance() / speed);
        } else {
            let pos = float_data.start + float_data.lerp(time.elapsed());
            transform.translation = Val2::px(pos.x, pos.y);
        }
    }
}

#[derive(Debug, Clone, Component)]
struct FloatsPositionUiData {
    start: Vec2,
    target: Vec2,
    move_start: Vec2,
    move_start_time: Duration,
    move_duration: Duration,
}

impl FloatsPositionUiData {
    pub fn new(start: Vec2) -> Self {
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

    pub fn lerp(&self, time: Duration) -> Vec2 {
        self.move_start
            .lerp(self.target, crate::easing::sine_in_out(self.t(time)))
    }

    pub fn move_distance(&self) -> f32 {
        self.move_start.distance(self.target)
    }
}
