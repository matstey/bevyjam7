use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update.in_set(AppSystems::Update).in_set(PausableSystems),),
    );
}

#[derive(Debug, Component)]
pub struct TimedImageChange {
    pub transition_time: Duration,
    pub next: Handle<Image>,
}

fn update(mut query: Query<(&TimedImageChange, &mut ImageNode)>, time: Res<Time>) {
    for (transition, mut image) in query.iter_mut() {
        if time.elapsed() > transition.transition_time && image.image != transition.next {
            image.image = transition.next.clone();
            info!("Background changed");
        }
    }
}
