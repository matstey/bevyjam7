use std::time::Duration;

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    games::{
        Game, GameData, GameInfo, GameState,
        pre_game::control_method::{ControlMethodAssets, control_method},
    },
    layout,
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout, TimeoutLabel},
};

mod balance;
mod control_method;
mod hint;

const GAME: Game = Game::Pre;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PreGameAssets>();
    app.add_plugins((hint::plugin, control_method::plugin));
    app.add_systems(OnEnter(GAME), spawn);
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
/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PreGameAssets {
    #[dependency]
    background1: Handle<Image>,
}

impl FromWorld for PreGameAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background1: assets.load("games/pre_game/background1.jpeg"),
        }
    }
}

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<PreGameState>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    control_assets: Res<ControlMethodAssets>,
    game_assets: Res<PreGameAssets>,
    data: Res<GameData>,
) {
    if let GameState::PreGame(info) = game_state.get() {
        state.reset(time.elapsed(), *info);

        commands
            .spawn((
                widget::ui_root("pre_game_ui"),
                DespawnOnExit(GAME), // When exiting this game despawn this entity
                DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
                Timeout::new(balance::COUNTDOWN),
                children![(
                    layout::grid_parent(),
                    ImageNode::new(game_assets.background1.clone()),
                    children![
                        (
                            layout::top_center(),
                            children![widget::header(format!("{}", info.kind)),]
                        ),
                        (layout::bottom_right(), children![TimeoutLabel],),
                        (
                            layout::bottom_left(),
                            children![control_method(info.controls, &control_assets)],
                        ),
                        (
                            layout::top_left(),
                            children![widget::label(format!("{}", data.round))],
                        )
                    ],
                )],
            ))
            .observe(timed_out);
    }
}

fn timed_out(
    _event: On<TimedOut>,
    state: Res<PreGameState>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_game: ResMut<NextState<Game>>,
) {
    let kind = state.info.kind;
    next_game_state.set(GameState::Game(kind));
    next_game.set(kind);
    info!("Pre game completed. Starting {}", kind);
}
