use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::games::cat_bonk::{CatBonkAssets, CatBonkState, balance};

#[derive(Debug, Default, Component)]
pub struct Cat {
    pub popup_delay: Duration,
}

pub fn cat(
    assets: &CatBonkAssets,
    state: &CatBonkState,
    pos: Vec2,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // use bevy random source?
    let mut rng = rand::rng();
    let max = state.run_time.as_secs_f64() * balance::MAX_SPAWN_MULTIPLIER;
    let delay = rng.random_range(0.0..max);

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 128, y: 60 }, 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("cat"),
        Transform::from_xyz(pos.x, pos.y, 1.0),
        Visibility::Hidden,
        Sprite::from_atlas_image(
            assets.cat.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Pickable::default(),
        Cat {
            popup_delay: Duration::from_secs_f64(delay),
        },
    )
}

// todo: pop up after random time period
pub fn update(
    time: Res<Time>,
    state: Res<CatBonkState>,
    cats: Query<(&Cat, &mut Visibility, &mut Sprite)>,
) {
    let elapsed = time.elapsed() - state.start_time;

    for (cat, mut visibility, mut sprite) in cats {
        if *visibility == Visibility::Hidden && elapsed > cat.popup_delay {
            visibility.toggle_visible_hidden();
        }

        if elapsed > cat.popup_delay + Duration::from_secs_f32(0.3)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = 1;
        }

        //todo: play sound
    }
}

pub fn on_hit(click: On<Pointer<Click>>, mut commands: Commands, mut state: ResMut<CatBonkState>) {
    commands.entity(click.entity).despawn();
    //todo: play sound
    //todo: hit effect?

    state.hit_count += 1;
}
