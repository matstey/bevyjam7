use bevy::prelude::*;

use crate::{
    backgrounds::BackgroundAssets,
    games::GameData,
    screens::{self, Screen},
    theme::widget,
};

const SCREEN: Screen = Screen::PostGame;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SCREEN), spawn);
}

pub fn spawn(mut commands: Commands, assets: Res<BackgroundAssets>, data: ResMut<GameData>) {
    commands.spawn((
        widget::ui_root("Game Over"),
        ImageNode::new(assets.background3.clone()),
        DespawnOnExit(SCREEN),
        children![
            widget::header("Game Over"),
            widget::label(format!("Passed: {}", data.passed)),
            widget::label(format!("Failed: {}", data.failed)),
            widget::button("Again?", screens::enter_loading_or_gameplay_screen),
        ],
    ));
}
