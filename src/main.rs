use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use three_card::{game::{CardDeck, GamePlugin}, networking::MyNetworkingPlugin, read_local_inputs, setup, AppState, Config};

/*
    Currently based on Matchbox Guide:
 - https://johanhelsing.studio/posts/extreme-bevy
 - https://github.com/johanhelsing/matchbox

 */

 // TODO: use PeerId's to create a see that used commonly used for RNG across each Client

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
            GgrsPlugin::<Config>::default(),
            MyNetworkingPlugin,
    ))
        // .rollback_component_with_clone::<Transform>()
        .rollback_resource_with_clone::<CardDeck>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .init_state::<AppState>()
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(ReadInputs, read_local_inputs)
        .run();
}
