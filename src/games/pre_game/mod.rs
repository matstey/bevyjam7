use std::time::Duration;

use bevy::prelude::*;

use crate::{
    games::{Game, GameState},
    screens::Screen,
    theme::widget,
};

const GAME: Game = Game::PreGame;
const COUNTDOWN: Duration = Duration::from_secs(5);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(Update, (update, update_countdown).run_if(in_state(GAME)));
    app.init_resource::<PreGameState>();
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct PreGameState {
    pub start_time: Duration,
}

impl PreGameState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
    }
}

/// Anything with this component will have its `Text` set to the countdown time
#[derive(Debug, Default, Component)]
#[require(Text)]
struct PreGameCountdown;

/// A system to spawn the example level
pub fn spawn(mut commands: Commands, mut state: ResMut<PreGameState>, time: Res<Time>) {
    state.reset(time.elapsed());

    commands.spawn((
        widget::ui_root("Pre Game Level"),
        DespawnOnExit(GAME), // When exiting this game despawn this entity
        DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
        children![
            widget::header("Pre Game"),
            (widget::label("0"), PreGameCountdown)
        ],
    ));
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(
    state: Res<PreGameState>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_game: ResMut<NextState<Game>>,
) {
    if let GameState::PreGame(info) = game_state.get() {
        if time.elapsed() - state.start_time > COUNTDOWN {
            next_game_state.set(GameState::Game(info.kind));
            next_game.set(info.kind);
            info!("Pre game completed. Starting {}", info.kind);
        }
    }
}

/// Update anything with the `PreGameCountdown` component to display the current countdown
fn update_countdown(
    mut query: Query<&mut Text, With<PreGameCountdown>>,
    state: Res<PreGameState>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        let elapsed = time.elapsed() - state.start_time;
        let countdown = if elapsed < COUNTDOWN {
            (COUNTDOWN - elapsed).as_secs_f32().ceil() as u32
        } else {
            0
        };
        text.0 = format!("{}", countdown)
    }
}
