use bevy::{color::palettes::css, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    controls::progress_bar::ProgressBar,
    float::Floats,
    games::{GameData, pre_game::GAME},
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<ThermometerAssets>();
    app.add_systems(OnEnter(GAME), spawn);
    app.add_systems(
        Update,
        (update.in_set(AppSystems::Update).in_set(PausableSystems),),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ThermometerAssets {
    #[dependency]
    foreground: Handle<Image>,
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for ThermometerAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            foreground: assets.load("games/pre_game/thermometer_foreground.png"),
            background: assets.load("games/pre_game/thermometer_background.png"),
        }
    }
}

#[derive(Debug, Component)]
pub struct Thermometer;

fn spawn(mut commands: Commands, assets: Res<ThermometerAssets>) {
    commands.spawn((
        widget::ui_root("heath"),
        ZIndex(3),
        DespawnOnExit(Screen::Gameplay),
        DespawnOnExit(GAME),
        Floats,
        children![
            (
                Node {
                    right: px(0),
                    position_type: PositionType::Absolute,
                    width: px(120.5),
                    height: px(299.5),
                    ..default()
                },
                ImageNode::new(assets.background.clone()),
                Pickable::IGNORE,
            ),
            (
                Node {
                    right: px(0),
                    position_type: PositionType::Absolute,
                    width: px(120.5),
                    height: px(299.5),
                    ..default()
                },
                Thermometer,
                ProgressBar {
                    color: css::RED.into(),
                    color_texture: assets.foreground.clone(),
                    ..default()
                },
                Pickable::IGNORE,
            )
        ],
    ));
}

fn update(mut query: Query<&mut ProgressBar, With<Thermometer>>, data: Res<GameData>) {
    //const MIN_THERMOMETER: f32 = 0.3;
    for mut progress_bar in query.iter_mut() {
        progress_bar.progress = data.fever_grade_nominal();
    }
}
