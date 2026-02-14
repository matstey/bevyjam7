//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{backgrounds::BackgroundAssets, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (open_main_menu, spawn));
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn spawn(mut commands: Commands, assets: Res<BackgroundAssets>) {
    commands.spawn((
        widget::ui_root("title"),
        DespawnOnExit(Screen::Title), // When exiting the top level game despawn this entity
        ImageNode::new(assets.background1.clone()),
    ));
}
