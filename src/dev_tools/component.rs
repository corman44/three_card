use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct InfoTimer(pub Timer);

impl Default for InfoTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(5), TimerMode::Repeating))
    }
}