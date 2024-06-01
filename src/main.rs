use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use three_card::{game::networking::{systems::Config, MyNetworkingPlugin}, move_players, read_local_inputs, setup, spawn_players};

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
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
    ))
        .add_plugins(MyNetworkingPlugin)
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (setup, spawn_players))
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_players)
        .run();
}
