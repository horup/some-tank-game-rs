use bevy::prelude::*;

mod components;
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;


fn startup_system(mut commands:Commands, mut new_game_writer:EventWriter<NewGameEvent>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    new_game_writer.send(NewGameEvent {
        map_size:8
    });
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::build();
    // add plugins
    builder.add_plugins(DefaultPlugins);

    // add events
    builder.add_event::<NewGameEvent>();

    // add systems
    builder.add_startup_system(startup_system.system())
    .add_system(game_system.system())
    .add_system(tilemap_render_system.system())
    .add_system(movement_system.system())
    .add_system_to_stage(CoreStage::PreUpdate, camera_system.system());

    builder.run();
}