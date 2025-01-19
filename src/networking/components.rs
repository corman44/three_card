use bevy::prelude::*;

use bevy_matchbox::{prelude::PeerId, MatchboxSocket};
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

#[derive(Debug, Resource)]
pub struct GameRoom {
    pub socket: MatchboxSocket,
    pub id: u64,
}

impl GameRoom {
    pub fn new(sock: MatchboxSocket, id: u64) -> Self {
        GameRoom { socket: sock, id: id }
    }

    pub fn send(&mut self, action: PlayerCommand) {
        let msg = bitcode::encode(&action);
        let peers = self.socket.connected_peers().collect::<Vec<_>>();
        for p in peers.iter() {
            self.socket.get_channel_mut(0)
                .expect("unable to get channel: 0")
                .send(msg.clone().into(), *p);
        }
    }

    pub fn receive(&mut self) -> Vec<(PeerId, PlayerCommand)> {
        self.socket
            .get_channel_mut(0)
            .expect("unable to get channel: 0")
            .receive()
            .iter()
            .filter_map(|(id,msg)| {
                if let Ok(msg) = bitcode::decode::<PlayerCommand>(&msg) {
                    Some((*id, msg))
                } else {
                    None
                }
            })
            .collect::<Vec<(PeerId,PlayerCommand)>>()
    }

    pub fn is_ok(&self) -> bool {
        self.socket.connected_peers().count() > 0
    }
}

pub trait IntoU64 {
    fn into_u64(&self) -> u64;
}

impl IntoU64 for PeerId {
    fn into_u64(&self) -> u64 {
        self.0.as_u64_pair().0 ^ self.0.as_u64_pair().1
    }
}
