//! The game's menus and transitions between them.

mod main;
mod pause;
mod settings;

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<MenuAssets>();
    app.init_state::<Menu>();

    app.add_plugins((main::plugin, settings::plugin, pause::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Settings,
    Pause,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MenuAssets {
    #[dependency]
    pub button: Handle<Image>,
    #[dependency]
    pub bgm: Handle<AudioSource>,
    #[dependency]
    pub font: Handle<Font>,
}

impl FromWorld for MenuAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            button: assets.load("images/button.png"),
            bgm: assets.load("audio/menu_bgm.ogg"),
            font: assets.load("fonts/RockSalt-Regular.ttf"),
        }
    }
}
