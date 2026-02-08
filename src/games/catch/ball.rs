use bevy::prelude::*;

use crate::games::catch::{CatchAssets, CatchState};

pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<CatchState>,
    time: Res<Time>,
    assets: Res<CatchAssets>,
) {
    if (time.elapsed() - state.last_release) > state.release_freq {
        state.last_release = time.elapsed();
        if let Some(root) = state.root {
            let ball_entity = commands.spawn(ball(&assets)).id();
            commands.entity(root).add_child(ball_entity);
        }
    }
}

pub fn ball(assets: &CatchAssets) -> impl Bundle {
    (
        Name::new("Ball"),
        Sprite::from_image(assets.ball.clone()),
        Transform::from_scale(Vec2::splat(0.25).extend(1.0)), // TODO: Randomize position
                                                              // TODO: Destroy after time comp
                                                              // TODO: Scale over time comp
    )
}
