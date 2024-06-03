pub mod game;
pub mod menus;
pub mod networking;

use bevy::{prelude::*, render::{camera::ScalingMode, texture::DefaultImageSampler}, ui::widget::UiImageSize, utils::HashMap, window::PrimaryWindow};
use bevy_ggrs::{AddRollbackCommandExtension, LocalInputs, LocalPlayers, PlayerInputs};
use bevy_matchbox::matchbox_socket::PeerId;
use game::components::LPTableCards;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

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
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(100.);
    commands.spawn(camera_bundle);

    // Spawn Background
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(r"Backgrounds\background_1.png").into(),
            transform: Transform {
                translation: Vec3::new(0.,0.,0.),
                scale: Vec3::new(0.4, 0.4, 1.),
                ..default()
            },
            ..default()
        }
    );
}

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}
