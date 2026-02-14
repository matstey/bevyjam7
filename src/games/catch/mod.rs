use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    games::{
        Game, GameControlMethod, GameInfo, GameResult, NextGame,
        catch::{
            ball::Ball,
            glove::{Glove, glove},
        },
    },
    screens::Screen,
    timeout::Timeout,
};

mod balance;
mod ball;
mod glove;

const GAME: Game = Game::Catch;

pub(super) fn plugin(app: &mut App) {
    // Register our assets to be loaded when the application is loading
    app.load_resource::<CatchAssets>();

    // Register our spawn system to be triggered when this game is selected
    app.add_systems(OnEnter(GAME), spawn);

    // Register all systems that are to be run when this game is active
    app.add_systems(
        Update,
        (update, ball::spawn, update_catch)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(GAME)),
    );

    // Register a basic data structure that we can use to track data for this game
    app.init_resource::<CatchState>();
}

pub const fn get_info() -> GameInfo {
    GameInfo {
        kind: GAME,
        controls: GameControlMethod::Wasd,
        hint: "Catch",
        color: 0xFFFFFFFF,
    }
}

/// All data representing the current state of this game
#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct CatchState {
    pub start_time: Duration,
    pub run_time: Duration,
    pub caught: u32,
    pub dropped: u32,
    pub release_freq: Duration,
    pub last_release: Duration,
    pub root: Option<Entity>,
}

impl CatchState {
    /// Called when starting this game to make sure the data is reset
    /// Assuming that is what we want.
    pub fn reset(&mut self, start_time: Duration, root: Entity) {
        self.start_time = start_time;
        self.run_time = balance::GAME_DURATION;
        self.caught = 0;
        self.dropped = 0;
        self.release_freq = balance::DROP_FREQ; // TODO: Scale based on rounds/time
        self.last_release = start_time;
        self.root = Some(root);
    }
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CatchAssets {
    #[dependency]
    glove: Handle<Image>,
    #[dependency]
    ball: Handle<Image>,
}

impl FromWorld for CatchAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            glove: assets.load("games/catch/glove.png"),
            ball: assets.load("games/catch/ball.png"),
        }
    }
}

/// A system to spawn the example level
pub fn spawn(
    mut commands: Commands,
    assets: Res<CatchAssets>,
    mut state: ResMut<CatchState>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let root = commands
        .spawn((
            Name::new("catch_level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(GAME), // When exiting this game despawn this entity
            DespawnOnExit(Screen::Gameplay), // When exiting the top level game despawn this entity
            Timeout::default(),
            children![glove(
                balance::GLOVE_SPEED,
                &assets,
                &mut meshes,
                &mut materials
            ),],
        ))
        .id();
    state.reset(time.elapsed(), root);
}

/// Just a simple system that transitions us to the next game after some time
pub fn update(state: Res<CatchState>, time: Res<Time>, mut tx: MessageWriter<NextGame>) {
    if time.elapsed() - state.start_time > state.run_time {
        tx.write(NextGame::from_result(if state.caught > 3 {
            GameResult::Passsed
        } else {
            GameResult::Failed
        }));
        info!("Next game");
    }
}

pub fn update_catch(
    mut commands: Commands,
    mut state: ResMut<CatchState>,
    ball_query: Query<(Entity, &Transform, &Ball)>,
    glove_query: Query<(&Transform, &Glove)>,
) {
    for (glove_transform, glove) in glove_query.iter() {
        for (ball_entity, ball_transform, ball) in ball_query.iter() {
            let distance = (glove_transform.translation - ball_transform.translation).length();

            if distance
                < (glove.radius * glove_transform.scale.x) + (ball_transform.scale.x * ball.radius)
            {
                state.caught += 1;
                commands.entity(ball_entity).despawn();
                info!("Caught ball!",);
            }
        }
    }
}
