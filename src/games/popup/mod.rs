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
    color::color_u32,
    games::{
        Game, GameControlMethod, GameInfo, GameResult, NextGame,
        camera::{self, shake::CameraShakeConfig},
    },
    screens::Screen,
    theme::widget,
    timeout::{TimedOut, Timeout, TimeoutBar},
};

mod balance;
mod popup_window;

const GAME: Game = Game::Popup;

pub(super) fn plugin(app: &mut App) {
    // Register our assets to be loaded when the application is loading
    app.load_resource::<PopupAssets>();

    // Register our spawn system to be triggered when this game is selected
    app.add_systems(OnEnter(GAME), spawn);

    // Register all systems that are to be run when this game is active
    app.add_systems(
        Update,
        (update, popup_window::update)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<PopupState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Mouse,
        hint: "Close",
        color: 0x5555FFFF,
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct PopupState {
    pub start_time: Duration,
    pub remaining: u32,
}

impl PopupState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration) {
        self.start_time = start_time;
        self.remaining = 5;
    }
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PopupAssets {
    #[dependency]
    pub popups: Vec<Handle<Image>>,
    #[dependency]
    pub background: Handle<Image>,
    #[dependency]
    pub notify_sound: Handle<AudioSource>,
    #[dependency]
    pub close_sound: Handle<AudioSource>,
}

impl FromWorld for PopupAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            popups: vec![
                assets.load_with_settings(
                    "games/popup/popup_large1.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_large2.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_large3.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_long1.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_long2.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_long3.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_wide1.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_wide2.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "games/popup/popup_wide3.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ],
            background: assets.load_with_settings(
                "games/popup/popup_background1.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            notify_sound: assets.load("games/popup/notify.ogg"),
            close_sound: assets.load("games/popup/close.ogg"),
        }
    }
}

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    assets: Res<PopupAssets>,
    mut state: ResMut<PopupState>,
    time: Res<Time>,
) {
    state.reset(time.elapsed());
    commands.spawn((
        DespawnOnExit(GAME),             // When exiting this game despawn this entity
        DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
        Timeout::default(),
        Camera2d,
        CameraShakeConfig::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 225.0,
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
        .with_children(|parent| {
            for i in 0..state.remaining {
                parent
                    .spawn(popup_window::popup_window(&assets, i + 1))
                    .observe(popup_window::on_hit);
            }
        })
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
        .add_children(&[level]);

    commands
        .spawn((
            widget::ui_root("popup_ui"),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            Timeout::new(balance::GAME_DURATION),
            children![TimeoutBar::from_foreground_color(color_u32(
                get_info().color
            ))],
        ))
        .observe(timed_out);
}

fn timed_out(_event: On<TimedOut>, mut tx: MessageWriter<NextGame>) {
    tx.write(NextGame::from_result(GameResult::Failed));
    info!("timeout - next game");
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(state: Res<PopupState>, mut tx: MessageWriter<NextGame>) {
    if state.remaining == 0 {
        tx.write(NextGame::from_result(GameResult::Passsed));
        info!("all targets closed - next game");
    }
}
