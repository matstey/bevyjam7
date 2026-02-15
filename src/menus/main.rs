//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    app,
    audio::music,
    menus::{Menu, MenuAssets},
    screens::{self},
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, assets: Res<MenuAssets>) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        music(assets.bgm.clone()),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::header(app::NAME, assets.font.clone()),
            widget::image_button(
                "Play",
                screens::enter_loading_or_gameplay_screen,
                assets.button.clone(),
                assets.font.clone()
            ),
            widget::image_button(
                "Settings",
                open_settings_menu,
                assets.button.clone(),
                assets.font.clone()
            ),
            widget::image_button("Exit", exit_app, assets.button.clone(), assets.font.clone()),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::header(app::NAME, assets.font.clone()),
            widget::image_button(
                "Play",
                screens::enter_loading_or_gameplay_screen,
                assets.button.clone(),
                assets.font.clone()
            ),
            widget::image_button(
                "Settings",
                open_settings_menu,
                assets.button.clone(),
                assets.font.clone()
            ),
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
