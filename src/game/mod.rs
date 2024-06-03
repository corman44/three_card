pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{add_players, deal_cards};

use crate::{spawn_players, CardDeck};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CardDeck>()
            .add_systems(Startup, (spawn_players, add_players))
            .add_systems(PostStartup, deal_cards)
            ;
    }
}