use std::{fmt, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::screens::Screen;

mod balance;
mod camera;
mod cat_bonk;
mod catch;
mod example;
mod lobster;
mod popup;
mod pre_game;
mod rain;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Game>();
    app.init_state::<GameState>();
    app.init_resource::<GameData>();
    app.add_message::<NextGame>();
    // Has to be in post update to make sure any request for the next level are processed before the next loop starts
    app.add_systems(PostUpdate, spawn_next);

    // Register all mini games here
    app.add_plugins((
        camera::plugin,
        pre_game::plugin,
        example::plugin,
        catch::plugin,
        cat_bonk::plugin,
        popup::plugin,
        lobster::plugin,
        rain::plugin,
    ));
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum Game {
    #[default]
    None,
    Pre,
    Example,
    Catch,
    CatBonk,
    Popup,
    Lobster,
    Rain,
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
                Game::Catch => "Catch",
                Game::CatBonk => "CatBonk",
                Game::Popup => "Popup",
                Game::Lobster => "Lobster",
                Game::Rain => "Rain",
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
    pub color: u32,
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
    pub round: usize,
    pub elapsed: Duration,
    pub passed: usize,
    pub failed: usize,
    pub level: usize,
    pub random: usize,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            round: 1,
            elapsed: Default::default(),
            passed: 0,
            failed: 0,
            level: 0,
            random: 0,
        }
    }
}

impl GameData {
    fn apply_result(&mut self, result: GameResult, delta: Duration) {
        self.round += 1;
        match result {
            GameResult::Passsed => {
                self.passed += 1;
            }
            GameResult::Failed => {
                self.failed += 1;
            }
        };
        self.elapsed += delta;
        self.level = self.round / balance::ROUNDS_PER_LEVEL;
    }

    pub fn fever_grade(&self) -> f32 {
        // 0.0 -> MAX_FEVER
        (self.failed as f32).clamp(0.0, balance::MAX_FEVER)
    }

    pub fn fever_grade_nominal(&self) -> f32 {
        self.fever_grade() / balance::MAX_FEVER
    }

    pub fn dead(&self) -> bool {
        self.fever_grade() >= balance::MAX_FEVER
    }

    pub fn reset(&mut self) {
        self.passed = 0;
        self.failed = 0;
        self.round = 1;
        self.elapsed = Duration::default();
        let mut rng = rand::rng();
        self.level = 0;
        self.random = rng.random_range(0..20);
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

/// A system that triggers the first game to spawn
pub fn spawn_first(
    mut game_data: ResMut<GameData>,
    mut next_game: ResMut<NextState<Game>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_data.reset();
    next_game.set(Game::Pre);
    next_game_state.set(GameState::PreGame(GameTransitionInfo {
        next: get_info(Game::Rain),
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
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let current = *game.get(); // Store the current game so we only every transition once but still process all messages
    for game in rx.read() {
        let next_game_kind = match current {
            Game::None => Game::CatBonk,
            Game::Example => Game::CatBonk,
            Game::Catch => Game::CatBonk,
            Game::CatBonk => Game::Popup,
            Game::Popup => Game::Lobster,
            Game::Lobster => Game::Rain,
            Game::Rain => Game::CatBonk,
            Game::Pre => todo!(), // If this get hit something has gone wrong
        };

        game_data.apply_result(game.result, Duration::from_secs(5)); // TODO: Actually time passed between games?

        if game_data.dead() {
            next_screen.set(Screen::PostGame);
            info!("Game over");
        } else {
            next_game.set(Game::Pre);
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
}

const fn get_info(game: Game) -> GameInfo {
    match game {
        Game::None => todo!(),
        Game::Pre => todo!(), // If this get hit something has gone wrong
        Game::Example => example::get_info(),
        Game::Catch => catch::get_info(),
        Game::CatBonk => cat_bonk::get_info(),
        Game::Popup => popup::get_info(),
        Game::Lobster => lobster::get_info(),
        Game::Rain => rain::get_info(),
    }
}
