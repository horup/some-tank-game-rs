use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};

mod components;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;

mod resources;
use resources::*;

mod factory;
pub use factory::*;

mod plugins;
pub use plugins::*;


fn startup_system(mut commands:Commands, mut new_game_writer:EventWriter<NewGameEvent>, mut rapier:ResMut<RapierConfiguration>) {
    rapier.gravity.x = 0.0;
    rapier.gravity.y = 0.0;
    new_game_writer.send(NewGameEvent::default());
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::build();
    
    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(TilemapPlugin::default())
    .add_plugin(SpriteBuilderPlugin::default())
    .add_plugin(EventsPlugin::default());

    // add resources
    builder.insert_resource(Textures::default());
    builder.insert_resource(Mouse::default());

    // add events
    builder.add_event::<NewGameEvent>();

    // add systems
    builder.add_startup_system(startup_system.system())
    .add_startup_system(load_textures_system.system())
    .add_system(input_system.system())
    .add_system(mouse_input_system.system())
    .add_system(game_system.system())
    .add_system(movement_system.system())
    .add_system(drag_system.system())
    .add_system(turret_system.system())
    .add_system(camera_system.system())
    .add_system(bot_system.system())
    .add_system(test_system.system())
    .add_system(projectile_system.system().after("physics"))
    .add_system(physics_system.system().label("physics"))
    .add_system(health_system.system())
    .add_system(tank_system.system())
    .add_system(effect_system.system());

    builder.run();
}