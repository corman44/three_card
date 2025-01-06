pub mod game;
pub mod menus;
pub mod networking;
pub mod dev_tools;

use bevy::{core_pipeline::bloom::Bloom, prelude::*, utils::HashMap};
use bevy_matchbox::matchbox_socket::PeerId;

// Input mapping:
//  - number -> selecting card for ready to play (only allow selecting of same value card)
//  - Enter -> for attempting to play the selected cards
//  - P -> Pickup cards
const INPUT_1: u64 = 1 << 0;
const INPUT_2: u64 = 1 << 1;
const INPUT_3: u64 = 1 << 2;
const INPUT_4: u64 = 1 << 3;
const INPUT_5: u64 = 1 << 4;
const INPUT_6: u64 = 1 << 5;
const INPUT_7: u64 = 1 << 6;
const INPUT_8: u64 = 1 << 7;
const INPUT_9: u64 = 1 << 8;
const INPUT_ENTER: u64 = 1 << 9;
const INPUT_PICKUPPILE: u64 = 1 << 10;
const INPUT_PICKUPDECK: u64 = 1 << 11;

#[derive(Debug, Default, States, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppState{
    #[default]
    WaitingForPlayers,
    PlayersMatched,
    GameStart,
    Playing,
    GameEnd,
}

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Spawn Camera
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::NATURAL,
    ));

    // Spawn Background
    commands.spawn(
        Sprite {
            image: asset_server.load(r"Backgrounds\background_1.png").into(),
            ..default()
        }
    );
}

// pub fn read_local_inputs(
//     mut commands: Commands,
//     keys: Res<ButtonInput<KeyCode>>,
//     local_players: Res<LocalPlayers>,
// ) {
//     let mut local_inputs = HashMap::new();

//     // TODO: convert key presses to mouse clicks of objects
//     for handle in &local_players.0 {
//         let mut input = 0u64;

//         if keys.pressed(KeyCode::Digit1) {
//             input |= INPUT_1;
//         }
//         if keys.pressed(KeyCode::Digit2) {
//             input |= INPUT_2;
//         }
//         if keys.pressed(KeyCode::Digit3) {
//             input |= INPUT_3;
//         }
//         if keys.pressed(KeyCode::Digit4) {
//             input |= INPUT_4;
//         }
//         if keys.pressed(KeyCode::Digit5) {
//             input |= INPUT_5;
//         }
//         if keys.pressed(KeyCode::Digit6) {
//             input |= INPUT_6;
//         }
//         if keys.pressed(KeyCode::Digit7) {
//             input |= INPUT_7;
//         }
//         if keys.pressed(KeyCode::Digit8) {
//             input |= INPUT_8;
//         }
//         if keys.pressed(KeyCode::Digit9) {
//             input |= INPUT_9;
//         }
//         if keys.pressed(KeyCode::Enter) {
//             input |= INPUT_ENTER;
//         }
//         if keys.pressed(KeyCode::KeyP) {
//             input |= INPUT_PICKUPPILE;
//         }
//         if keys.pressed(KeyCode::KeyD) {
//             input |= INPUT_PICKUPDECK;
//         }

//         local_inputs.insert(*handle, input);
//     }

//     commands.insert_resource(LocalInputs::<Config>(local_inputs));
// }

