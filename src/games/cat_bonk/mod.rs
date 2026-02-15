use std::time::Duration;

use rand::seq::index;

use bevy::{
    app::Propagate,
    camera::ScalingMode,
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    color::color_u32,
    games::{
        Game, GameControlMethod, GameData, GameInfo, GameResult, NextGame,
        camera::{self, shake::CameraShakeConfig},
    },
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout, TimeoutBar},
};

mod balance;
pub mod cat;
pub mod level;
pub mod weapon;

const GAME: Game = Game::CatBonk;

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CatBonkAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    weapon: Handle<Image>,
    #[dependency]
    cat: Handle<Image>,
    #[dependency]
    hit_sound: Handle<AudioSource>,
    #[dependency]
    cat_sounds: Vec<Handle<AudioSource>>,
    #[dependency]
    cat_hit_sounds: Vec<Handle<AudioSource>>,
    #[dependency]
    bgm: Handle<AudioSource>,
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
                "games/cat/cat1-sheet.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            hit_sound: assets.load("games/cat/hit.ogg"),
            cat_sounds: vec![
                assets.load("games/cat/cat1.ogg"),
                assets.load("games/cat/cat2.ogg"),
                assets.load("games/cat/cat3.ogg"),
            ],
            cat_hit_sounds: vec![
                assets.load("games/cat/cat_hit1.ogg"),
                assets.load("games/cat/cat_hit2.ogg"),
                assets.load("games/cat/cat_hit3.ogg"),
            ],
            bgm: assets.load("games/cat/bgm.ogg"),
        }
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct CatBonkState {
    pub start_time: Duration,
    pub target_count: usize,
    pub hit_count: usize,
}

impl CatBonkState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration, level: usize) {
        self.start_time = start_time;
        self.target_count = balance::NUM_CATS + (level * balance::CATS_PER_LEVEL);
        self.hit_count = 0;
    }
}

pub fn spawn(
    mut commands: Commands,
    gamedata: Res<GameData>,
    assets: Res<CatBonkAssets>,
    mut state: ResMut<CatBonkState>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
) {
    state.reset(time.elapsed(), gamedata.level);

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
        DespawnOnExit(GAME),             // When exiting this game despawn this entity
        DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
        Camera2d,
        CameraShakeConfig::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 450.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        camera::RENDERLAYER_GAME,
    ));

    commands
        .spawn((
            widget::ui_root("CatBonk UI"),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            Timeout::new(balance::GAME_DURATION),
            children![TimeoutBar::from_foreground_color(color_u32(
                get_info().color
            ))],
        ))
        .observe(timed_out);

    let level = commands
        .spawn(level::level(&assets))
        .with_children(|parent| {
            // spawn cats at random locations
            for spawn_index in indices {
                parent
                    .spawn(cat::cat(
                        &assets,
                        cat_spawns[spawn_index],
                        &mut texture_atlas_layouts,
                    ))
                    .observe(cat::on_hit);
            }
        })
        .id();

    //Spawn weapon
    let weapon = commands.spawn(weapon::weapon(&assets)).id();

    commands
        .spawn((
            Name::new("root"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(GAME),
            DespawnOnExit(Screen::Gameplay),
            Propagate(camera::RENDERLAYER_GAME),
            AudioPlayer(assets.bgm.clone()),
        ))
        .add_children(&[level, weapon]);
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CatBonkAssets>();
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(
        Update,
        (
            update,
            level::update,
            weapon::update,
            cat::update,
            weapon::update_weapon_hit.run_if(input_just_pressed(MouseButton::Left)),
        )
            .run_if(in_state(GAME)),
    );
    app.init_resource::<CatBonkState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Mouse,
        hint: "Bonk",
        color: 0x279CD8FF,
    }
}

fn timed_out(_event: On<TimedOut>, mut tx: MessageWriter<NextGame>) {
    tx.write(NextGame::from_result(GameResult::Failed));
    info!("timeout - next game");
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(state: Res<CatBonkState>, mut tx: MessageWriter<NextGame>) {
    if state.hit_count >= state.target_count {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("all targets hit - next game");
    }
}
