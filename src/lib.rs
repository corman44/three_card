pub mod game;
pub mod menus;
pub mod networking;

use bevy::{prelude::*, render::{camera::ScalingMode, texture::DefaultImageSampler}, ui::widget::UiImageSize, utils::HashMap, window::PrimaryWindow};
use bevy_ggrs::{AddRollbackCommandExtension, LocalInputs, LocalPlayers, PlayerInputs};
use bevy_matchbox::matchbox_socket::PeerId;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Clone, Debug, Default)]
pub enum CardVal {
    Hearts_Ace,
    Hearts_2,
    Hearts_3,
    Hearts_4,
    Hearts_5,
    Hearts_6,
    Hearts_7,
    Hearts_8,
    Hearts_9,
    Hearts_10,
    Hearts_Jack,
    Hearts_Queen,
    Hearts_King,
    Diamonds_Ace,
    Diamonds_2,
    Diamonds_3,
    Diamonds_4,
    Diamonds_5,
    Diamonds_6,
    Diamonds_7,
    Diamonds_8,
    Diamonds_9,
    Diamonds_10,
    Diamonds_Jack,
    Diamonds_Queen,
    Diamonds_King,
    Clubs_Ace,
    Clubs_2,
    Clubs_3,
    Clubs_4,
    Clubs_5,
    Clubs_6,
    Clubs_7,
    Clubs_8,
    Clubs_9,
    Clubs_10,
    Clubs_Jack,
    Clubs_Queen,
    Clubs_King,
    Spades_Ace,
    Spades_2,
    Spades_3,
    Spades_4,
    Spades_5,
    Spades_6,
    Spades_7,
    Spades_8,
    Spades_9,
    Spades_10,
    Spades_Jack,
    Spades_Queen,
    Spades_King,
    #[default]
    Init,
}

#[derive(Resource, Debug)]
pub struct CardDeck{
    pub cards: Vec<CardVal>,
}

impl Default for CardDeck {
    fn default() -> Self {
        // ordered deck of cards
        Self { cards: vec![CardVal::Hearts_Ace, CardVal::Hearts_2, CardVal::Hearts_3, CardVal::Hearts_4, CardVal::Hearts_5, CardVal::Hearts_6, CardVal::Hearts_7, CardVal::Hearts_8, CardVal::Hearts_9, CardVal::Hearts_10, CardVal::Hearts_Jack, CardVal::Hearts_Queen, CardVal::Hearts_King, CardVal::Diamonds_Ace, CardVal::Diamonds_2, CardVal::Diamonds_3, CardVal::Diamonds_4, CardVal::Diamonds_5, CardVal::Diamonds_6, CardVal::Diamonds_7, CardVal::Diamonds_8, CardVal::Diamonds_9, CardVal::Diamonds_10, CardVal::Diamonds_Jack, CardVal::Diamonds_Queen, CardVal::Diamonds_King, CardVal::Clubs_Ace, CardVal::Clubs_2, CardVal::Clubs_3, CardVal::Clubs_4, CardVal::Clubs_5, CardVal::Clubs_6, CardVal::Clubs_7, CardVal::Clubs_8, CardVal::Clubs_9, CardVal::Clubs_10, CardVal::Clubs_Jack, CardVal::Clubs_Queen, CardVal::Clubs_King, CardVal::Spades_Ace, CardVal::Spades_2, CardVal::Spades_3, CardVal::Spades_4, CardVal::Spades_5, CardVal::Spades_6, CardVal::Spades_7, CardVal::Spades_8, CardVal::Spades_9, CardVal::Spades_10, CardVal::Spades_Jack, CardVal::Spades_Queen, CardVal::Spades_King] }
    }
}

#[derive(Debug, Default, States, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppState{
    #[default]
    WaitingForPlayers,
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
                scale: Vec3::new(0.2, 0.2, 1.),
                ..default()
            },
            ..default()
        }
    );

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(0.02, 0.02, 1.),
                ..default()
            },
            ..default()
        }
    );

}

pub fn spawn_players(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // let win_width = window.get_single().unwrap().width();
    // let win_height = window.get_single().unwrap().height();


}

pub fn move_players(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_seconds();
        transform.translation += move_delta.extend(0.);
    }
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

#[derive(Component)]
pub struct Player {
    handle: usize
}
