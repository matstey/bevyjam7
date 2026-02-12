use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    games::{
        GameState,
        pre_game::{GAME, balance},
    },
    theme::widget,
};

#[derive(Debug, Copy, Clone, Component)]
pub struct Hint {
    display_time: Duration,
    destroy_time: Duration,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(
        Update,
        (update.in_set(AppSystems::Update).in_set(PausableSystems),),
    );
}

pub fn spawn(mut commands: Commands, game_state: Res<State<GameState>>, time: Res<Time>) {
    if let GameState::PreGame(game) = game_state.get() {
        info!("Hint spawn");
        commands.spawn((
            widget::ui_root("Hint"),
            Visibility::Hidden,
            ZIndex(2),
            children![widget::header(game.next.hint)],
            Hint {
                display_time: time.elapsed() + balance::HINT_DISPLAY_TIME,
                destroy_time: time.elapsed()
                    + balance::HINT_DISPLAY_TIME
                    + balance::HINT_DESTROY_TIME,
            },
        ));
    }
}

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility, &Hint)>,
    time: Res<Time>,
) {
    for (entity, mut visability, hint) in query.iter_mut() {
        if *visability == Visibility::Hidden && time.elapsed() > hint.display_time {
            *visability = Visibility::Visible;
            info!("Hint Visible");
        } else if time.elapsed() > hint.destroy_time {
            commands.entity(entity).despawn();
            info!("Hint despawn");
        }
    }
}
