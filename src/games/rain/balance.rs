use std::time::Duration;

pub const GAME_DURATION: Duration = Duration::from_secs(5);

// Seconds out of shelter before you lose
pub const MAX_WET_TIME: f32 = 1.0;
// Distance from the center of the umbrella before counting as not in shelter
pub const SHELTER_THRESHOLD: f32 = 22.0;

pub const UMBRELLA_MAX_VELOCITY: f32 = 30.0;
pub const PLAYER_MOVEMENT_SPEED: f32 = 42.0;
pub const LEVEL_MULTIPLIER: f32 = 1.07;
