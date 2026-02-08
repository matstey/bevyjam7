use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

#[derive(Debug, Clone, Copy, Component)]
pub struct DespawnAfter {
    start_time: Duration,
    lifespan: Duration,
}

impl DespawnAfter {
    pub fn new(start_time: Duration, lifespan: Duration) -> Self {
        Self {
            start_time,
            lifespan,
        }
    }

    pub fn expiry(&self) -> Duration {
        self.start_time + self.lifespan
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_despawn
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

fn update_despawn(mut commands: Commands, query: Query<(Entity, &DespawnAfter)>, time: Res<Time>) {
    for (entity, despawn) in query.iter() {
        if time.elapsed() > despawn.expiry() {
            commands.entity(entity).despawn();
        }
    }
}
