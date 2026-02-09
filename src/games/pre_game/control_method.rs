use crate::{asset_tracking::LoadResource, float::Floats, games::GameControlMethod, theme};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<ControlMethodAssets>();
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ControlMethodAssets {
    #[dependency]
    keyboard: Handle<Image>,
    #[dependency]
    mouse: Handle<Image>,
    #[dependency]
    wasd: Handle<Image>,
}

impl FromWorld for ControlMethodAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            keyboard: assets.load("games/pre_game/keyboard.png"),
            mouse: assets.load("games/pre_game/mouse.png"),
            wasd: assets.load("games/pre_game/wasd.png"),
        }
    }
}

pub fn control_method(method: GameControlMethod, assets: &ControlMethodAssets) -> impl Bundle {
    (
        Name::new("control_method"),
        ImageNode::new(asset_from_method(method, assets)).with_color(theme::palette::HEADER_TEXT),
        Node {
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        },
        Outline {
            width: px(6),
            offset: px(6),
            color: theme::palette::HEADER_TEXT,
        },
        Floats,
    )
}

fn asset_from_method(method: GameControlMethod, assets: &ControlMethodAssets) -> Handle<Image> {
    match method {
        GameControlMethod::Wasd => assets.wasd.clone(),
        GameControlMethod::Mouse => assets.mouse.clone(),
        GameControlMethod::Keyboard => assets.wasd.clone(),
    }
}
