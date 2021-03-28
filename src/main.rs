use bevy::prelude::*;

mod components;

mod systems;
use systems::*;

// https://github.com/bevyengine/bevy/tree/v0.4.0/examples/2d
fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(init.system())
    .add_system(game.system())
    .run();
}
