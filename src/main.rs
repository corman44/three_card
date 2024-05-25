use bevy::prelude::*;
use three_card::spawn_hello_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_hello_world)
        .run()
        ;
}
