use rand::Rng;
use std::time::Duration;

use bevy::{
    app::Propagate,
    camera::ScalingMode,
    image::{ImageLoaderSettings, ImageSampler},
    input::{common_conditions::input_just_pressed},
    prelude::*,

};

use crate::float::Floats;
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

mod balance;
mod oyster;

const GAME: Game = Game::Lobster;

pub(super) fn plugin(app: &mut App) {
    // Register our assets to be loaded when the application is loading
    app.load_resource::<LobsterAssets>();

    // Register our spawn system to be triggered when this game is selected
    app.add_systems(OnEnter(GAME), spawn);

    // Register all systems that are to be run when this game is active
    app.add_systems(
        Update,
        (oyster::update,
            oyster::try_grab.run_if(input_just_pressed(KeyCode::Space)),
            )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<LobsterState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Keyboard,
        hint: "Grab",
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct LobsterState {
    pub start_time: Duration,
    pub caught: Option<bool>,
}

impl LobsterState {
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.caught = None;
    }
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LobsterAssets {
    #[dependency]
    pub lobster: Handle<Image>,
    #[dependency]
    pub shrimp: Handle<Image>,
    #[dependency]
    pub oyster: Handle<Image>,
    #[dependency]
    pub pearl: Handle<Image>,
    #[dependency]
    pub background: Handle<Image>,
}

impl FromWorld for LobsterAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            lobster: assets.load_with_settings(
                "games/lobster/lobster.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            shrimp: assets.load_with_settings(
                "games/lobster/shrimp.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            oyster: assets.load_with_settings(
                "games/lobster/oyster.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            pearl: assets.load_with_settings(
                "games/lobster/pearl.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            background: assets.load_with_settings(
                "games/lobster/background.png",
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
    assets: Res<LobsterAssets>,
    mut state: ResMut<LobsterState>,
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
                viewport_height: 450.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        camera::RENDERLAYER_GAME,
    ));

    let level = commands
        .spawn((
            Name::new("background"),
            Transform::default(),
            Visibility::default(),
            Sprite::from_image(assets.background.clone()),
        ))
        .id();

    let mut rng = rand::rng();

    for i in 0..4 {
        let x = ((i as f32 * 180.0) + 110.0) - 400.0;
        let y = rng.random_range(100.0..200.0);

        commands.spawn((
            Name::new("shrimp"),
            Transform::from_xyz(x, y, 1.0),
            Visibility::default(),
            Sprite::from_image(assets.shrimp.clone()),
            Floats,
            ChildOf(level),
        ));
    }

    let oyster = commands
        .spawn(oyster::oyster(
            &assets,
            Vec2 { x: 240.0, y: -90.0 },
            &mut texture_atlas_layouts,
        )).id();

    commands
        .spawn((
            Name::new("root"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(GAME),
            DespawnOnExit(Screen::Gameplay),
            Propagate(camera::RENDERLAYER_GAME),
        ))
        .add_children(&[level, oyster]);

    commands
        .spawn((
            widget::ui_root("lobster_ui"),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            Timeout::new(balance::GAME_DURATION),
            children![TimeoutLabel],
        ))
        .observe(timed_out);
}

fn timed_out(_event: On<TimedOut>, mut tx: MessageWriter<NextGame>, state: Res<LobsterState>) {
    if let Some(caught) = state.caught && caught {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("grabbed - next game");
    }
    else {
        tx.write(NextGame::from_result(GameResult::Failed));
        info!("failed - next game");
    }
}

