use bevy::{prelude::*, window::WindowResolution};
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use three_card::{game::GamePlugin, move_players, networking::MyNetworkingPlugin, read_local_inputs, setup, spawn_players, CardDeck, Config};

/*
    Currently based on Matchbox Guide:
 - https://johanhelsing.studio/posts/extreme-bevy
 - https://github.com/johanhelsing/matchbox

 */


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    resolution: (934., 523.).into(),
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
            // GgrsPlugin::<Config>::default(),
            // MyNetworkingPlugin,
    ))
        // .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_players)
        // .add_systems(ReadInputs, read_local_inputs)
        // .add_systems(GgrsSchedule, move_players)
        .run();
}
