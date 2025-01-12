use bevy::prelude::*;

use bitcode::{Encode, Decode};

#[derive(Debug, Encode, Decode, Default)]
pub enum ActionType {
    PickupPile,
    #[default]
    PickupDeck,
    PlayCards,
}

/// informs peers of action
#[derive(Debug, Encode, Decode, Default)]
pub struct PlayerCommand {
    pub action: ActionType,
    pub data: Option<Vec<u8>>,
}
