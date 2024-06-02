pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Debug, Component, Default)]
pub struct Active(pub bool);

#[derive(Debug, Clone, Component, Default)]
pub struct Card;

#[derive(Bundle, Clone, Default)]
pub struct CardBundle {
    card: Card,
    sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct UICardBundle {
    active: Active,
    card_bundle: CardBundle,
}