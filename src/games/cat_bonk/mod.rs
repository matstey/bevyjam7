use rand::prelude::*;
use rand::seq::index;
use std::time::Duration;

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    games::{Game, GameControlMethod, GameInfo, GameResult, NextGame},
    screens::Screen,
    theme::widget,
};

pub mod cat;
pub mod level;
pub mod weapon;

const GAME: Game = Game::CatBonk;

/// Anything with this component will have its `Text` set to the countdown time
#[derive(Debug, Default, Component)]
#[require(Text)]
struct CatBonkCountdown;

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CatBonkAssets {
    #[dependency]
    background: Handle<Image>,
    weapon: Handle<Image>,
    cat: Handle<Image>,
}

impl FromWorld for CatBonkAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load_with_settings(
                "games/cat/background.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            weapon: assets.load_with_settings(
                "games/cat/hammer.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            cat: assets.load_with_settings(
                "games/cat/cat1.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct CatBonkState {
    pub start_time: Duration,
    pub run_time: Duration,
    pub target_count: u32,
    pub hit_count: u32,
}

impl CatBonkState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.run_time = Duration::from_secs(5);

        // todo: scale from difficulty
        self.target_count = 3;
        self.hit_count = 0;
    }
}

pub fn spawn(
    mut commands: Commands,
    assets: Res<CatBonkAssets>,
    mut state: ResMut<CatBonkState>,
    time: Res<Time>,
) {
    state.reset(time.elapsed());

    // hardcoded list of possible cat spawn locations...
    // could have done something smarter here, but theres not too many locations
    let cat_spawns = [
        Vec2 { x: -30.0, y: 120.0 },
        Vec2 { x: 150.0, y: 125.0 },
        Vec2 { x: 350.0, y: 128.0 },
        Vec2 { x: -90.0, y: 44.0 },
        Vec2 { x: 98.0, y: 50.0 },
        Vec2 { x: 290.0, y: 56.0 },
        Vec2 { x: -32.0, y: -63.0 },
        Vec2 { x: 150.0, y: -55.0 },
        Vec2 { x: 350.0, y: -52.0 },
        Vec2 {
            x: -84.0,
            y: -136.0,
        },
        Vec2 { x: 98.0, y: -130.0 },
        Vec2 {
            x: 290.0,
            y: -124.0,
        },
    ];

    let mut rng = rand::rng();
    let indices = index::sample(&mut rng, cat_spawns.len(), state.target_count as usize);

    commands.spawn((
        widget::ui_root("CatBonk UI"),
        DespawnOnExit(GAME), // When exiting this game despawn this entity
        DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
        children![
            widget::header("BONK"),
            (widget::label("0"), CatBonkCountdown)
        ],
    ));

    commands
        .spawn(level::level(&assets))
        .with_children(|parent| {
            // spawn cats at random locations
            for spawn_index in indices {
                parent
                    .spawn(cat::cat(&assets, cat_spawns[spawn_index]))
                    .observe(cat::on_hit);
            }
        });

    //Spawn weapon
    commands.spawn(weapon::weapon(&assets));
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CatBonkAssets>();
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(
        Update,
        (update, level::update, weapon::update, update_countdown).run_if(in_state(GAME)),
    );
    app.init_resource::<CatBonkState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Mouse,
    }
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(state: Res<CatBonkState>, time: Res<Time>, mut tx: MessageWriter<NextGame>) {
    if state.hit_count >= state.target_count {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("all targets hit - next game");
    }

    if time.elapsed() - state.start_time > state.run_time {
        tx.write(NextGame::from_result(GameResult::Failed));
        info!("timeout - next game");
    }
}

/// Update anything with the `ExampleCountdown` component to display the current countdown
fn update_countdown(
    mut query: Query<&mut Text, With<CatBonkCountdown>>,
    state: Res<CatBonkState>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        let elapsed = time.elapsed() - state.start_time;
        let countdown = if elapsed < state.run_time {
            (state.run_time - elapsed).as_secs_f32().ceil() as u32
        } else {
            0
        };
        text.0 = format!("{}", countdown)
    }
}
