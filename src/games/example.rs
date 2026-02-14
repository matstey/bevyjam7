use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    games::{Game, GameControlMethod, GameInfo, GameResult, NextGame},
    screens::Screen,
    theme::widget,
    timeout::Timeout,
};

const GAME: Game = Game::Example;

pub(super) fn plugin(app: &mut App) {
    // Register our assets to be loaded when the application is loading
    app.load_resource::<ExampleAssets>();

    // Register our spawn system to be triggered when this game is selected
    app.add_systems(OnEnter(GAME), spawn);

    // Register all systems that are to be run when this game is active
    app.add_systems(
        Update,
        (update, update_countdown)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<ExampleState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Wasd,
        hint: "Go",
        color: 0xFFFFFFFF,
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct ExampleState {
    pub start_time: Duration,
    pub run_time: Duration,
}

impl ExampleState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.run_time = Duration::from_secs(5);
    }
}

/// Anything with this component will have its `Text` set to the countdown time
#[derive(Debug, Default, Component)]
#[require(Text)]
struct ExampleCountdown;

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ExampleAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for ExampleAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("games/duck/Fluffing A Duck.ogg"),
        }
    }
}

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    _assets: Res<ExampleAssets>,
    mut state: ResMut<ExampleState>,
    time: Res<Time>,
) {
    state.reset(time.elapsed());

    commands.spawn((
        widget::ui_root("Example Level"),
        DespawnOnExit(GAME), // When exiting this game despawn this entity
        DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
        Timeout::default(),
        children![
            widget::header("Example Game"),
            (widget::label("0"), ExampleCountdown)
        ],
    ));
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(state: Res<ExampleState>, time: Res<Time>, mut tx: MessageWriter<NextGame>) {
    if time.elapsed() - state.start_time > state.run_time {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("Next game");
    }
}

/// Update anything with the `ExampleCountdown` component to display the current countdown
fn update_countdown(
    mut query: Query<&mut Text, With<ExampleCountdown>>,
    state: Res<ExampleState>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        let elapsed = time.elapsed() - state.start_time;
        let countdown = if elapsed < state.run_time {
            (state.run_time - elapsed).as_secs_f32().ceil() as u32
        } else {
            0
        };
        text.0 = format!("{}", countdown)
    }
}
