use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BackgroundAssets>();
}

/// Used to track all assets for this game
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BackgroundAssets {
    #[dependency]
    pub background1: Handle<Image>,
    #[dependency]
    pub background2: Handle<Image>,
    #[dependency]
    pub background3: Handle<Image>,
}

impl FromWorld for BackgroundAssets {
    /// Load all assets we want for this game
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background1: assets.load("backgrounds/background1.jpeg"),
            background2: assets.load("backgrounds/background2.jpeg"),
            background3: assets.load("backgrounds/background3.jpeg"),
        }
    }
}

impl BackgroundAssets {
    pub const COUNT: usize = 3;

    pub fn index(&self, index: usize) -> Handle<Image> {
        match index % Self::COUNT {
            0 => self.background1.clone(),
            1 => self.background2.clone(),
            2 => self.background3.clone(),
            _ => self.background1.clone(),
        }
    }
}
