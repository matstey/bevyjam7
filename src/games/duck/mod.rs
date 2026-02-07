use bevy::prelude::*;

use crate::games::Game;

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
