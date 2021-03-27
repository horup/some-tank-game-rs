use bevy::prelude::*;

mod components;

mod systems;
use systems::*;


fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(init.system())
    .add_system(game.system())
    .run();
}
