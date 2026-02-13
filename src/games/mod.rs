use std::{fmt, time::Duration};

use bevy::{color::palettes::css, prelude::*};

use crate::{
    asset_tracking::LoadResource, controls::progress_bar::ProgressBar, float::Floats,
    screens::Screen, theme::widget,
};

mod balance;
mod camera;
mod cat_bonk;
mod catch;
mod duck;
mod example;
mod lobster;
mod popup;
mod pre_game;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GameAssets>();
    app.init_state::<Game>();
    app.init_state::<GameState>();
    app.init_resource::<GameData>();
    app.add_message::<NextGame>();
    // Has to be in post update to make sure any request for the next level are processed before the next loop starts
    app.add_systems(PostUpdate, spawn_next);
    app.add_systems(OnEnter(Screen::Gameplay), spawn_health);
    app.add_systems(Update, update_health_bar);

    // Register all mini games here
    app.add_plugins((
        camera::plugin,
        pre_game::plugin,
        example::plugin,
        duck::plugin,
        catch::plugin,
        cat_bonk::plugin,
        popup::plugin,
        lobster::plugin,
    ));
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum Game {
    #[default]
    None,
    Pre,
    Example,
    Duck,
    Catch,
    CatBonk,
    Popup,
    Lobster,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Game::None => "None",
                Game::Pre => "PreGame",
                Game::Example => "Example",
                Game::Duck => "Duck",
                Game::Catch => "Catch",
                Game::CatBonk => "CatBonk",
                Game::Popup => "Popup",
                Game::Lobster => "Lobster",
            }
        )
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    None,
    PreGame(GameTransitionInfo),
    Game(Game),
}

/// Global game state updated after each game completes
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GameInfo {
    pub kind: Game,
    pub controls: GameControlMethod,
    pub hint: &'static str,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GameTransitionInfo {
    pub next: GameInfo,
    pub last: Option<GameResult>,
}

#[allow(unused)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameControlMethod {
    #[default]
    Wasd,
    Mouse,
    Keyboard,
    Space,
    //Keys(Vec<KeyCode>),
}

impl fmt::Display for GameControlMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wasd => "WASD",
                Self::Mouse => "Mouse",
                Self::Keyboard => "Keyboard",
                Self::Space => "Space",
            }
        )
    }
}

/// Global game state updated after each game completes
#[derive(Debug, Copy, Clone, Resource)]
pub struct GameData {
    pub health: f32,
    pub round: u32,
    pub elapsed: Duration,
    pub passed: usize,
    pub failed: usize,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            health: Default::default(),
            round: 1,
            elapsed: Default::default(),
            passed: 0,
            failed: 0,
        }
    }
}

impl GameData {
    fn apply_result(&mut self, result: GameResult, delta: Duration) {
        self.round += 1;
        match result {
            GameResult::Passsed => {
                self.passed += 1;
                self.adjust_health(balance::PASSED_REWARD);
            }
            GameResult::Failed => {
                self.failed += 1;
                self.adjust_health(-balance::FAILED_COST);
            }
        };
        self.elapsed += delta;
    }

    fn adjust_health(&mut self, adjustment: f32) {
        self.health = (self.health + adjustment).clamp(0.0, balance::MAX_HEALTH);
    }

    pub fn fever_grade(&self) -> f32 {
        (self.failed as f32 / self.passed.max(1) as f32) + 1.0 // Range has to start at 1.0
    }
}

#[derive(Debug, Copy, Clone, Message)]
pub struct NextGame {
    pub result: GameResult,
}

impl NextGame {
    pub fn from_result(result: GameResult) -> Self {
        Self { result }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameResult {
    Passsed,
    Failed,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Passsed => "Passed",
                Self::Failed => "Failed",
            }
        )
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameAssets {
    #[dependency]
    thermometer: Handle<Image>,
    #[dependency]
    thermometer_background: Handle<Image>,
}

impl FromWorld for GameAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            thermometer: assets.load("thermometer.png"),
            thermometer_background: assets.load("thermometer_background.png"),
        }
    }
}

#[derive(Debug, Component)]
pub struct HealthBar;

fn spawn_health(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        widget::ui_root("heath"),
        ZIndex(3),
        DespawnOnExit(Screen::Gameplay),
        Floats,
        children![
            (
                Node {
                    right: px(0),
                    position_type: PositionType::Absolute,
                    width: px(120.5),
                    height: px(299.5),
                    ..default()
                },
                ImageNode::new(assets.thermometer_background.clone()),
                Pickable::IGNORE,
            ),
            (
                Node {
                    right: px(0),
                    position_type: PositionType::Absolute,
                    width: px(120.5),
                    height: px(299.5),
                    ..default()
                },
                HealthBar,
                ProgressBar {
                    color: css::RED.into(),
                    color_texture: assets.thermometer.clone(),
                    ..default()
                },
                Pickable::IGNORE,
            )
        ],
    ));
}

fn update_health_bar(mut query: Query<&mut ProgressBar, With<HealthBar>>, data: Res<GameData>) {
    for mut progress_bar in query.iter_mut() {
        progress_bar.progress = 0.35 + (data.fever_grade() / 10.0); // TODO: Decide how his logic should be. When do u die?
    }
}

/// A system that triggers the first game to spawn
pub fn spawn_first(
    mut next_game: ResMut<NextState<Game>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game.set(Game::Pre);
    next_game_state.set(GameState::PreGame(GameTransitionInfo {
        next: get_info(Game::CatBonk),
        last: None,
    }));
}

/// A system that triggers the next game to spawn when a `NextGame` message is sent
/// Logic here is current just sequential but can more more complex later
fn spawn_next(
    mut rx: MessageReader<NextGame>,
    game: Res<State<Game>>,
    mut game_data: ResMut<GameData>,
    mut next_game: ResMut<NextState<Game>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let current = *game.get(); // Store the current game so we only every transition once but still process all messages
    for game in rx.read() {
        let next_game_kind = match current {
            Game::None => Game::CatBonk,
            Game::Example => Game::CatBonk,
            Game::Duck => Game::Example,
            Game::Catch => Game::CatBonk,
            Game::CatBonk => Game::Popup,
            Game::Popup => Game::Lobster,
            Game::Lobster => Game::CatBonk,
            Game::Pre => todo!(), // If this get hit something has gone wrong
        };

        next_game.set(Game::Pre);
        game_data.apply_result(game.result, Duration::from_secs(5)); // TODO: Actually time passed between games?
        next_game_state.set(GameState::PreGame(GameTransitionInfo {
            next: get_info(next_game_kind),
            last: Some(game.result),
        }));

        info!(
            "Last game result {}. Next game {}.",
            game.result, next_game_kind
        );
    }
}

const fn get_info(game: Game) -> GameInfo {
    match game {
        Game::None => todo!(),
        Game::Pre => todo!(), // If this get hit something has gone wrong
        Game::Example => example::get_info(),
        Game::Duck => duck::get_info(),
        Game::Catch => catch::get_info(),
        Game::CatBonk => cat_bonk::get_info(),
        Game::Popup => popup::get_info(),
        Game::Lobster => lobster::get_info(),
    }
}
