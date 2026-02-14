use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};

use crate::{AppSystems, PausableSystems, controls::progress_bar::ProgressBar};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn, update, spawn_bar, update_bar)
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
pub struct TimeoutBar {
    pub foreground_color: Color,
    pub background_color: Color,
}

impl TimeoutBar {
    pub fn from_foreground_color(color: Color) -> Self {
        Self {
            foreground_color: color,
            ..default()
        }
    }
}

impl Default for TimeoutBar {
    fn default() -> Self {
        Self {
            foreground_color: css::BLUE_VIOLET.into(),
            background_color: Color::srgb(0.25, 0.25, 0.25),
        }
    }
}

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

fn spawn_bar(mut commands: Commands, query: Query<(Entity, &TimeoutBar), Added<TimeoutBar>>) {
    for (entity, timeout_bar) in query.iter() {
        commands.entity(entity).insert((
            Node {
                bottom: px(0),
                left: px(0),
                right: px(0),
                margin: UiRect::horizontal(Val::Auto),
                padding: UiRect::all(Val::Px(5.0)),
                display: Display::Block,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(timeout_bar.background_color),
            ProgressBar {
                color: timeout_bar.foreground_color,
                vertical: false,
                ..default()
            },
        ));
    }
}

// TODO: Support hierarchy where `Timeout` and label are not direct child/parent
fn update_bar(
    mut label_query: Query<(&ChildOf, &mut ProgressBar), With<TimeoutBar>>,
    timeout_query: Query<&TimeoutState>,
    time: Res<Time>,
) {
    for (parent, mut bar) in label_query.iter_mut() {
        if let Ok(state) = timeout_query.get(parent.0) {
            let elapsed = time.elapsed() - state.start_time;
            let countdown = if elapsed < state.run_time {
                elapsed.as_secs_f32().ceil() / state.run_time.as_secs_f32()
            } else {
                0.0
            };
            bar.progress = countdown;
        }
    }
}
