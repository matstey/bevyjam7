use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),),
    );
}

#[derive(Debug, Component)]
pub struct ShowAt {
    time: Duration,
}

impl ShowAt {
    pub fn from_duration(duration: Duration) -> Self {
        Self { time: duration }
    }
}

fn update(mut query: Query<(&ShowAt, &mut Visibility)>, time: Res<Time>) {
    for (show_at, mut visibility) in query.iter_mut() {
        if time.elapsed() > show_at.time {
            *visibility = Visibility::Visible;
        }
    }
}
