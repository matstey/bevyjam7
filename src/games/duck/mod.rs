use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    games::{Game, GameControlMethod, GameInfo, GameResult, NextGame},
};

mod animation;
pub mod level;
pub mod player;

const GAME: Game = Game::Duck;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, level::plugin, player::plugin));

    app.init_resource::<DuckState>();
    app.add_systems(OnEnter(GAME), reset);
    app.add_systems(
        Update,
        update
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct DuckState {
    pub start_time: Duration,
    pub run_time: Duration,
}

impl DuckState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.run_time = Duration::from_secs(5);
    }
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::WASD,
    }
}

pub fn reset(mut state: ResMut<DuckState>, time: Res<Time>) {
    state.reset(time.elapsed());
}

pub fn update(state: Res<DuckState>, time: Res<Time>, mut tx: MessageWriter<NextGame>) {
    if time.elapsed() - state.start_time > state.run_time {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("Next game");
    }
}
