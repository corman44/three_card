use std::{fmt::Debug, time::Duration};

use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct DebugTimer(pub Timer);

impl Default for DebugTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(5), TimerMode::Repeating))
    }
}