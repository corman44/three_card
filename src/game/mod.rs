pub mod menus;
pub mod networking;

use bevy::prelude::*;
use networking::MyNetworkingPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MyNetworkingPlugin);
    }
}