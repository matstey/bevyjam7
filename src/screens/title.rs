//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, menus::Menu, screens::Screen, theme::widget};

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TitleScreenAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for TitleScreenAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("games/pre_game/background1.jpeg"),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<TitleScreenAssets>();
    app.add_systems(OnEnter(Screen::Title), (open_main_menu, spawn));
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn spawn(mut commands: Commands, assets: Res<TitleScreenAssets>) {
    commands.spawn((
        widget::ui_root("title"),
        DespawnOnExit(Screen::Title), // When exiting the top level game despawn this entity
        ImageNode::new(assets.background.clone()),
    ));
}
