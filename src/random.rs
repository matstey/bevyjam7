use bevy::{prelude::*, window::PrimaryWindow};
use rand::{Rng, rngs::ThreadRng};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Random2dPosition>();
    app.add_systems(PreUpdate, update_area);
}

#[derive(Debug, Default, Resource)]
pub struct Random2dPosition {
    area: Vec2, // The area that we generate positions from
}

impl Random2dPosition {
    pub fn next(&self, padding: f32) -> Vec2 {
        let mut rng = rand::rng();
        Vec2::new(
            rng.random_range(-self.area.x + padding..self.area.x - padding),
            rng.random_range(-self.area.y + padding..self.area.y - padding),
        )
    }
}

fn update_area(mut random: ResMut<Random2dPosition>, window: Single<&Window, With<PrimaryWindow>>) {
    random.area = window.size() * 0.5; // Just sort half size as 0,0 is screen center
}

pub fn sign(rng: &mut ThreadRng) -> f32 {
    if rng.random_bool(0.5) { 1.0 } else { -1.0 }
}
