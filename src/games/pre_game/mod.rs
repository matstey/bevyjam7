use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    games::{
        Game, GameData, GameInfo, GameState,
        pre_game::control_method::{ControlMethodAssets, control_method},
    },
    layout,
    screens::Screen,
    theme::widget,
};

mod balance;
mod control_method;
mod hint;

const GAME: Game = Game::Pre;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hint::plugin, control_method::plugin));
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(
        Update,
        (update, update_countdown)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );
    app.init_resource::<PreGameState>();
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct PreGameState {
    pub start_time: Duration,
    pub info: GameInfo,
}

impl PreGameState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration, info: GameInfo) {
        self.start_time = start_time;
        self.info = info;
    }
}

/// Anything with this component will have its `Text` set to the countdown time
#[derive(Debug, Default, Component)]
#[require(Text)]
struct PreGameCountdown;

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<PreGameState>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    assets: Res<ControlMethodAssets>,
    data: Res<GameData>,
) {
    if let GameState::PreGame(info) = game_state.get() {
        state.reset(time.elapsed(), *info);

        commands.spawn((
            widget::ui_root("pre_game_ui"),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            children![(
                layout::grid_parent(),
                children![
                    (
                        layout::top_center(),
                        children![widget::header(format!("{}", info.kind)),]
                    ),
                    (
                        layout::bottom_right(),
                        children![(widget::label("0"), Node { ..default() }, PreGameCountdown)]
                    ),
                    (
                        layout::bottom_left(),
                        children![control_method(info.controls, &assets)]
                    ),
                    (
                        layout::top_left(),
                        children![widget::label(format!("{}", data.round + 1))]
                    )
                ],
            )],
        ));
    }
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(
    state: Res<PreGameState>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_game: ResMut<NextState<Game>>,
) {
    if time.elapsed() - state.start_time > balance::COUNTDOWN {
        let kind = state.info.kind;
        next_game_state.set(GameState::Game(kind));
        next_game.set(kind);
        info!("Pre game completed. Starting {}", kind);
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
        let countdown = if elapsed < balance::COUNTDOWN {
            (balance::COUNTDOWN - elapsed).as_secs_f32().ceil() as u32
        } else {
            0
        };
        text.0 = format!("{}", countdown)
    }
}
