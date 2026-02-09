use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::audio::sound_effect;
use crate::float::Floats;
use crate::games::{popup::PopupAssets, popup::PopupState, popup::balance};

#[derive(Debug, Default, Component)]
pub struct PopupWindow {
    close: Rect,
    popup_delay: Duration,
}

pub fn popup_window(assets: &PopupAssets, state: &PopupState, index: u32) -> impl Bundle {
    let mut rng = rand::rng();
    let idx = rng.random_range(0..assets.popups.len());
    let asset = assets.popups[idx].clone();

    let size = match idx {
        0..3 => Vec2 { x: 166.0, y: 138.0 },
        3..6 => Vec2 { x: 111.0, y: 164.0 },
        6..9 => Vec2 { x: 150.0, y: 55.0 },
        _ => Vec2::default(),
    };

    let screen = Vec2 { x: 400.0, y: 225.0 };
    let max = ((screen / 2.0) - (size / 2.0)) * 0.9;

    let x = rng.random_range(-max.x..max.x);
    let y = rng.random_range(-max.y..max.y);
    let close_loc = Rect::from_center_size((size / 2.0) - 8.0, Vec2::splat(17.0));

    let max = state.run_time.as_secs_f64() * balance::MAX_SPAWN_DELAY_MULTIPLIER;
    let delay = rng.random_range(0.0..max);

    println!("{} {}", size, close_loc.center());

    (
        Name::new("popup"),
        Transform::from_xyz(x, y, index as f32),
        Visibility::Hidden,
        Sprite::from_image(asset),
        Pickable::default(),
        PopupWindow {
            close: close_loc,
            popup_delay: Duration::from_secs_f64(delay),
        },
        Floats,
    )
}

pub fn on_hit(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    assets: Res<PopupAssets>,
    mut state: ResMut<PopupState>,
    query: Query<&GlobalTransform>,
    popup_query: Query<&PopupWindow>,
) {
    if let Ok(popup) = popup_query.get(click.entity)
        && let Some(world_pos) = click.hit.position
        && let Ok(transform) = query.get(click.entity)
    {
        // Convert world position to local position
        let local_pos = transform.affine().inverse().transform_point3(world_pos);
        let local_2d = Vec2 {
            x: local_pos.x,
            y: local_pos.y,
        };

        if popup.close.contains(local_2d) {
            commands.entity(click.entity).despawn();
            commands.spawn(sound_effect(assets.close_sound.clone()));
            state.remaining -= 1;
        }
    }
}

// todo: pop up after random time period
pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<PopupAssets>,
    state: Res<PopupState>,
    popups: Query<(&PopupWindow, &mut Visibility)>,
) {
    let elapsed = time.elapsed() - state.start_time;

    for (popup, mut visibility) in popups {
        if *visibility == Visibility::Hidden && elapsed > popup.popup_delay {
            visibility.toggle_visible_hidden();
            commands.spawn(sound_effect(assets.notify_sound.clone()));
        }
    }
}
