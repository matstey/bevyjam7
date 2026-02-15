use bevy::prelude::*;

use crate::{
    backgrounds::BackgroundAssets,
    games::GameData,
    menus::MenuAssets,
    screens::{self, Screen},
    theme::widget,
};

const SCREEN: Screen = Screen::PostGame;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SCREEN), spawn);
}

pub fn spawn(
    mut commands: Commands,
    assets: Res<BackgroundAssets>,
    data: ResMut<GameData>,
    menu_assets: Res<MenuAssets>,
) {
    commands.spawn((
        widget::ui_root("Game Over"),
        ImageNode::new(assets.index(data.random)),
        DespawnOnExit(SCREEN),
        children![
            widget::header("Game Over", menu_assets.font.clone()),
            widget::label(format!("Passed: {}", data.passed), menu_assets.font.clone()),
            widget::label(format!("Failed: {}", data.failed), menu_assets.font.clone()),
            widget::image_button(
                "Again?",
                screens::enter_loading_or_gameplay_screen,
                menu_assets.button.clone(),
                menu_assets.font.clone()
            ),
        ],
    ));
}
