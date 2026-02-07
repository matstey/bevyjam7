use bevy::prelude::*;

mod duck;
mod example;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Game {
    #[default]
    None,
    Example,
    Duck,
}

#[derive(Debug, Default, Copy, Clone, Message)]
pub struct NextGame;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Game>();
    app.add_message::<NextGame>();
    // Has to be in post update to make sure any request for the next level are processed before the next loop starts
    app.add_systems(PostUpdate, spawn_next);

    // Register all mini games here
    app.add_plugins((example::plugin, duck::plugin));
}

/// A system that triggers the first game to spawn
pub fn spawn_first(mut next_level: ResMut<NextState<Game>>) {
    next_level.set(Game::Example);
}

/// A system that triggers the next game to spawn when a `NextGame` message is sent
/// Logic here is current just sequential but can more more complex later
fn spawn_next(
    mut rx: MessageReader<NextGame>,
    game: Res<State<Game>>,
    mut next_game: ResMut<NextState<Game>>,
) {
    let current = game.get(); // Store the current game so we only every transition once but still process all messages
    for _ in rx.read() {
        next_game.set(match current {
            Game::None => Game::Example,
            Game::Example => Game::Duck,
            Game::Duck => Game::None,
        });
    }
}
