use std::{fmt::Display, time::Duration};

use bevy::prelude::*;

mod balance;
mod camera;
mod cat_bonk;
mod catch;
mod duck;
mod example;
mod lobster;
mod popup;
mod pre_game;

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

impl Display for Game {
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
    PreGame(GameInfo),
    Game(Game),
}

/// Global game state updated after each game completes
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GameInfo {
    pub kind: Game,
    pub controls: GameControlMethod,
    pub hint: &'static str,
}

#[allow(unused)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameControlMethod {
    #[default]
    Wasd,
    Mouse,
    Keyboard,
    //Keys(Vec<KeyCode>),
}

impl Display for GameControlMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wasd => "WASD",
                Self::Mouse => "Mouse",
                Self::Keyboard => "Keyboard",
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
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            health: Default::default(),
            round: 1,
            elapsed: Default::default(),
        }
    }
}

impl GameData {
    fn apply_result(&mut self, result: GameResult, delta: Duration) {
        self.round += 1;
        match result {
            GameResult::Passsed => self.adjust_health(balance::PASSED_REWARD),
            GameResult::Failed => self.adjust_health(-balance::FAILED_COST),
        };
        self.elapsed += delta;
    }

    fn adjust_health(&mut self, adjustment: f32) {
        self.health = (self.health + adjustment).clamp(0.0, balance::MAX_HEALTH);
    }

    pub fn fever_grade(&self) -> f32 {
        self.round as f32 // TODO: Added health
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

impl Display for GameResult {
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
        duck::plugin,
        catch::plugin,
        cat_bonk::plugin,
        popup::plugin,
        lobster::plugin,
    ));
}

/// A system that triggers the first game to spawn
pub fn spawn_first(
    mut next_game: ResMut<NextState<Game>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game.set(Game::Pre);
    next_game_state.set(GameState::PreGame(get_info(Game::CatBonk)));
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
        next_game_state.set(GameState::PreGame(get_info(next_game_kind)));

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
