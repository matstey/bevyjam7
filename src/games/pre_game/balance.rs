use std::time::Duration;

/// How long we are in pre game for
pub const COUNTDOWN: Duration = Duration::from_secs(2);

// How many seconds after the pre game starts do we show the hint
pub const HINT_DISPLAY_TIME: Duration = Duration::from_secs(1);
// How long does the hint display for
pub const HINT_DESTROY_TIME: Duration = Duration::from_secs(2);
