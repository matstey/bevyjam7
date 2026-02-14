//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    menus::Menu,
    screens::{self},
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::header("<Game Name Here>"),
            widget::button("Play", screens::enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::header("<Game Name Here>"),
            widget::button("Play", screens::enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
        ],
    ));
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
