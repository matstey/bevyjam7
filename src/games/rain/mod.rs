use std::time::Duration;

use bevy::{
    app::Propagate,
    camera::RenderTarget,
    camera::ScalingMode,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    color::color_u32,
    games::{
        Game, GameControlMethod, GameInfo, GameResult, NextGame,
        camera::{self, shake::CameraShakeConfig},
        rain::umbrella::Umbrella,
    },
    movement::TopDownMovementController,
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout, TimeoutBar},
};

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::games::GameData;

mod animation;
mod balance;
mod duck;
mod umbrella;

const GAME: Game = Game::Rain;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<RainAssets>();
    app.add_systems(OnEnter(GAME), (spawn, spawn_camera));
    app.add_systems(
        Update,
        (update, umbrella::update)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );
    app.add_systems(
        Update,
        (duck::trigger_step_sound_effect)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<RainState>();
    app.add_plugins(animation::plugin);
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Wasd,
        hint: "Shelter",
        color: 0xFFFFFFFF,
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct RainState {
    pub start_time: Duration,
    pub wetness: f32,
}

impl RainState {
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.wetness = 0.0;
    }
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RainAssets {
    #[dependency]
    pub duck: Handle<Image>,
    #[dependency]
    pub duck_wet: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
    #[dependency]
    pub umbrella: Handle<Image>,
    #[dependency]
    pub rain: Handle<Image>,
    #[dependency]
    pub ground: Handle<Image>,
}

impl FromWorld for RainAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            duck: assets.load_with_settings(
                "games/rain/duck_anim.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            duck_wet: assets.load_with_settings(
                "games/rain/duck_wet_anim.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("games/rain/steps-001.ogg"),
                assets.load("games/rain/steps-002.ogg"),
                assets.load("games/rain/steps-003.ogg"),
                assets.load("games/rain/steps-004.ogg"),
                assets.load("games/rain/steps-005.ogg"),
            ],
            umbrella: assets.load_with_settings(
                "games/rain/umbrella.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            rain: assets.load_with_settings(
                "games/rain/rain.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            ground: assets.load_with_settings(
                "games/rain/droplets.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct OuterCamera;

const RES_WIDTH: u32 = 200;
const RES_HEIGHT: u32 = 113;

pub fn spawn_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // This Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    // Fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // Render RENDERLAYER_GAME to the RT
    commands.spawn((
        DespawnOnExit(GAME),
        DespawnOnExit(Screen::Gameplay),
        Camera2d,
        CameraShakeConfig::default(),
        Camera {
            order: -2,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        Msaa::Off,
        camera::RENDERLAYER_GAME,
    ));

    // Outer camera to render the RT
    commands.spawn((
        DespawnOnExit(GAME),
        DespawnOnExit(Screen::Gameplay),
        Camera2d,
        Camera {
            order: -1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: RES_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        OuterCamera,
        camera::RENDERLAYER_OUTER,
    ));

    // Spawn the canvas
    commands.spawn((
        DespawnOnExit(GAME),
        DespawnOnExit(Screen::Gameplay),
        Sprite::from_image(image_handle),
        Canvas,
        camera::RENDERLAYER_OUTER,
    ));
}

pub fn spawn(
    mut commands: Commands,
    assets: Res<RainAssets>,
    mut state: ResMut<RainState>,
    time: Res<Time>,
    gamedata: Res<GameData>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    state.reset(time.elapsed());

    let level_multiplier = f32::powi(balance::LEVEL_MULTIPLIER, gamedata.level as i32);
    info!("level mult = {}", level_multiplier);

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 200, y: 120 }, 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let level = commands
        .spawn((
            Name::new("background"),
            Transform::default(),
            Visibility::default(),
            Sprite::from_atlas_image(
                assets.rain.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            AnimationIndices { first: 0, last: 3 },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            children![
                umbrella::umbrella(&assets, balance::UMBRELLA_MAX_VELOCITY * level_multiplier),
                duck::duck(
                    &assets,
                    &mut texture_atlas_layouts,
                    balance::PLAYER_MOVEMENT_SPEED * level_multiplier
                )
            ],
        ))
        .id();

    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 200, y: 14 }, 1, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let ground = commands
        .spawn((
            Name::new("ground"),
            Transform::from_xyz(0.0, -50.0, 1.0),
            Visibility::default(),
            Sprite::from_atlas_image(
                assets.ground.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            AnimationIndices { first: 0, last: 4 },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .id();

    commands
        .spawn((
            Name::new("root"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(GAME),
            DespawnOnExit(Screen::Gameplay),
            Propagate(camera::RENDERLAYER_GAME),
        ))
        .add_children(&[level, ground]);

    commands
        .spawn((
            widget::ui_root("rain_ui"),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            Timeout::new(balance::GAME_DURATION),
            children![TimeoutBar::from_foreground_color(color_u32(
                get_info().color
            ))],
        ))
        .observe(timed_out);
}

fn timed_out(_event: On<TimedOut>, mut tx: MessageWriter<NextGame>, state: Res<RainState>) {
    if state.wetness < 1.0 {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("success - next game");
    } else {
        tx.write(NextGame::from_result(GameResult::Failed));
        info!("failed - next game");
    }
}

fn update(
    time: Res<Time>,
    assets: Res<RainAssets>,
    mut state: ResMut<RainState>,
    player: Single<(&Transform, &mut Sprite), With<TopDownMovementController>>,
    umbrella: Single<&Transform, With<Umbrella>>,
) {
    let (player_transform, mut sprite) = player.into_inner();

    let umbrella_dist = f32::abs(player_transform.translation.x - umbrella.translation.x);
    let sheltered = umbrella_dist < balance::SHELTER_THRESHOLD;

    let prev_wet = state.wetness < 1.0;

    if !sheltered {
        state.wetness += time.delta_secs() * balance::MAX_WET_TIME;

        if !prev_wet && state.wetness > 1.0 {
            sprite.image = assets.duck_wet.clone();
        }
    }
}
