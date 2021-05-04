use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};

mod components;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;

mod resources;
use resources::*;

mod factory;
pub use factory::*;


fn startup_system(mut commands:Commands, mut new_game_writer:EventWriter<NewGameEvent>) {
    new_game_writer.send(NewGameEvent::default());
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::build();
    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default());

    // add resources
    builder.insert_resource(Textures::default());
    builder.insert_resource(Mouse::default());

    // add events
    builder.add_event::<NewGameEvent>();

    // add systems
    builder.add_startup_system(startup_system.system())
    .add_startup_system(load_textures_system.system())
    .add_system(input_system.system().before("physics"))
    .add_system(mouse_input_system.system().before("physics"))
    .add_system(game_system.system())
    .add_system(tilemap_render_system.system())
    
    .add_system(physics_system.system().label("physics"))
    .add_system(movement_system.system().after("physics"))
    .add_system(turret_system.system().after("physics"))
    .add_system(camera_system.system().after("physics"))
    .add_system(bot_system.system().before("physics"));

    builder.run();
}