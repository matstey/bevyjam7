use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn, update, spawn_label, update_label)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(EntityEvent)]
pub struct TimedOut {
    entity: Entity,
}

#[derive(Debug, Clone, Component)]
pub struct Timeout {
    time: Duration,
}

impl Timeout {
    pub fn new(time: Duration) -> Self {
        Self { time }
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            time: Duration::from_secs(2),
        }
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct TimeoutLabel;

// TODO: ADD LABEL FOR COUNTDOWN

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TimeoutState {
    pub start_time: Duration,
    pub run_time: Duration,
    pub timed_out: bool,
}

impl TimeoutState {
    fn new(start_time: Duration, run_time: Duration) -> Self {
        Self {
            start_time,
            run_time,
            timed_out: false,
        }
    }
}

fn spawn(
    mut commands: Commands,
    query: Query<(Entity, &Timeout), Added<Timeout>>,
    time: Res<Time>,
) {
    for (entity, timeout) in query.iter() {
        commands
            .entity(entity)
            .insert(TimeoutState::new(time.elapsed(), timeout.time));
    }
}

fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TimeoutState), With<Timeout>>,
    time: Res<Time>,
) {
    for (entity, mut state) in query.iter_mut() {
        if !state.timed_out && time.elapsed() - state.start_time > state.run_time {
            state.timed_out = true;
            commands.trigger(TimedOut { entity });
        }
    }
}

fn spawn_label(mut commands: Commands, query: Query<Entity, (Added<TimeoutLabel>, Without<Text>)>) {
    for entity in query.iter() {
        commands.entity(entity).insert(widget::label("0"));
    }
}

// TODO: Support hierarchy where `Timeout` and label are not direct child/parent
fn update_label(
    mut label_query: Query<(&ChildOf, &mut Text), With<TimeoutLabel>>,
    timeout_query: Query<&TimeoutState>,
    time: Res<Time>,
) {
    for (parent, mut text) in label_query.iter_mut() {
        if let Ok(state) = timeout_query.get(parent.0) {
            let elapsed = time.elapsed() - state.start_time;
            let countdown = if elapsed < state.run_time {
                (state.run_time - elapsed).as_secs_f32().ceil() as u32
            } else {
                0
            };
            text.0 = format!("{}", countdown);
        }
    }
}
