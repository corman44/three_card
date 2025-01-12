pub mod game;
pub mod menus;
pub mod networking;
pub mod dev_tools;

use bevy::{core_pipeline::bloom::Bloom, prelude::*};

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

