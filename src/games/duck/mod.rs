use bevy::prelude::*;

use crate::games::{Game, GameControlMethod, GameInfo};

mod animation;
pub mod level;
mod movement;
pub mod player;

const GAME: Game = Game::Duck;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::WASD,
    }
}
