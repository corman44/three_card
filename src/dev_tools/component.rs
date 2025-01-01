use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct InfoTimer(pub Timer);

impl Default for InfoTimer {
    fn default() -> Self {
        Self(Timer::default())
    }
}