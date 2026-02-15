use std::time::Duration;

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    backgrounds::BackgroundAssets,
    games::{
        Game, GameData, GameInfo, GameResult, GameState,
        pre_game::control_method::{ControlMethodAssets, control_method},
    },
    layout,
    menus::MenuAssets,
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout},
    transition::TimedImageChange,
    visibility::ShowAt,
};

mod balance;
mod control_method;
mod hint;
mod thermometer;

const GAME: Game = Game::Pre;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PreGameAssets>();
    app.add_plugins((hint::plugin, control_method::plugin, thermometer::plugin));
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
    pass_background: Handle<Image>,
    #[dependency]
    fail_background: Handle<Image>,
    #[dependency]
    bgm: Handle<AudioSource>,
}

impl FromWorld for PreGameAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            pass_background: assets.load("games/pre_game/pass.jpeg"),
            fail_background: assets.load("games/pre_game/fail.jpeg"),
            bgm: assets.load("games/pre_game/bgm.ogg"),
        }
    }
}

fn image_from_result(
    result: Option<GameResult>,
    pre_game_assets: &Res<PreGameAssets>,
    background_assets: &Res<BackgroundAssets>,
) -> Handle<Image> {
    if let Some(result) = result {
        match result {
            GameResult::Passsed => pre_game_assets.pass_background.clone(),
            GameResult::Failed => pre_game_assets.fail_background.clone(),
        }
    } else {
        background_assets.background1.clone()
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
    background_assets: Res<BackgroundAssets>,
    data: Res<GameData>,
    menu_assets: Res<MenuAssets>,
) {
    if let GameState::PreGame(info) = game_state.get() {
        state.reset(time.elapsed(), info.next);

        info!(
            "Fever grade: {} ({})",
            data.fever_grade(),
            data.fever_grade_nominal()
        );

        info!("Level: {}", data.level);

        commands
            .spawn((
                widget::ui_root("pre_game_ui"),
                DespawnOnExit(GAME), // When exiting this game despawn this entity
                DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
                Timeout::new(balance::COUNTDOWN),
                AudioPlayer(game_assets.bgm.clone()),
                children![(
                    layout::grid_parent(),
                    children![
                        (
                            ImageNode::new(image_from_result(
                                info.last,
                                &game_assets,
                                &background_assets
                            )),
                            TimedImageChange {
                                transition_time: time.elapsed() + Duration::from_millis(500),
                                next: background_assets.index(data.round + data.random),
                            },
                            ZIndex(-1),
                        ),
                        (
                            layout::center(),
                            children![(
                                control_method(info.next.controls, &control_assets),
                                Visibility::Hidden,
                                ShowAt::from_duration(time.elapsed() + Duration::from_millis(500))
                            )],
                        ),
                        (
                            layout::top_left(),
                            children![widget::label(
                                format!("{}", data.round),
                                menu_assets.font.clone()
                            )],
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
