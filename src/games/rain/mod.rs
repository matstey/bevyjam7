use std::time::Duration;

use bevy::{
    app::Propagate,
    camera::ScalingMode,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    games::{
        Game, GameControlMethod, GameInfo, GameResult, NextGame,
        camera::{self, shake::CameraShakeConfig},
    },
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout, TimeoutLabel},
};

use crate::animation::{AnimationIndices, AnimationTimer};

mod balance;
mod umbrella;

const GAME: Game = Game::Rain;

pub(super) fn plugin(app: &mut App) {
    // Register our assets to be loaded when the application is loading
    app.load_resource::<RainAssets>();

    // Register our spawn system to be triggered when this game is selected
    app.add_systems(OnEnter(GAME), spawn);

    // Register all systems that are to be run when this game is active
    app.add_systems(
        Update,
        (update, umbrella::update)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<RainState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Wasd,
        hint: "Shelter",
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct RainState {
    pub start_time: Duration,
}

impl RainState {
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
    }
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RainAssets {
    #[dependency]
    pub duck: Handle<Image>,
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
                "games/rain/duck.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
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

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    assets: Res<RainAssets>,
    mut state: ResMut<RainState>,
    time: Res<Time>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    state.reset(time.elapsed());

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
                viewport_height: 113.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        camera::RENDERLAYER_GAME,
    ));

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
                umbrella::umbrella(&assets),
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
            children![TimeoutLabel],
        ))
        .observe(timed_out);
}

fn timed_out(_event: On<TimedOut>, mut tx: MessageWriter<NextGame>, state: Res<RainState>) {
    tx.write(NextGame::from_result(GameResult::Failed));
    info!("failed - next game");
}

fn update() {}
