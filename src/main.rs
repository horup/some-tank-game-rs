use bevy::prelude::*;

mod components;

mod systems;
use systems::*;

// https://github.com/bevyengine/bevy/tree/v0.4.0/examples/2d
fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(init_system.system())
    .add_system(tilemap_render_system.system())
    .add_system(movement_system.system())
    .run();
}
