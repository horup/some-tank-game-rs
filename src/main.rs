use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};

mod components;
use bevy_rapier2d::physics::{RapierConfiguration};
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;

mod resources;
use resources::*;
/*
mod factory;
pub use factory::*;*/

mod plugins;
pub use plugins::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    InBetweenGames,
    InGame
}

fn startup_system(mut new_game_writer:EventWriter<NewGameEvent>, mut rapier:ResMut<RapierConfiguration>) {
    rapier.gravity.x = 0.0;
    rapier.gravity.y = 0.0;
    rapier.time_dependent_number_of_timesteps = true;
    new_game_writer.send(NewGameEvent::default());
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::build();
    let window = WindowDescriptor {
        title: "Blueprint 3.0".to_string(),
        width: 640.0,
        height: 480.0,
        vsync: false,
        ..Default::default()
    };
    builder.insert_resource(window);

    // add state
    builder.add_state(AppState::InBetweenGames);

    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPluginCustom)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(TilemapPlugin::default())
    .add_plugin(SpriteBuilderPlugin::default())
    .add_plugin(EventsPlugin::default());
    
    // add resources
    builder.insert_resource(Textures::default())
    .insert_resource(Mouse::default())
    .insert_resource(Game::default())
    .insert_resource(Hud::default());

    // add events
    builder.add_event::<NewGameEvent>();
    
    

    // add startup systems
    builder
    .add_startup_system(startup_system.system())
    .add_startup_system(hud_initialization_system.system())
    .add_startup_system(load_textures_system.system());

    // add always on systems
    builder
    .add_system_to_stage(CoreStage::PreUpdate,game_tick_system.system())
    .add_system(game_system.system())
    .add_system(hud_system.system())
    .add_system(camera_system.system());

    // add in game update systems
    builder
    .add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(input_system.system())
        .with_system(mouse_input_system.system())
        .with_system(drag_system.system())
        .with_system(turret_system.system())
        .with_system(bot_system.system())
        .with_system(bot_sensor_system.system())
        .with_system(projectile_system.system().after("physics"))
        .with_system(physics_system.system().label("physics"))
        .with_system(health_system.system())
        .with_system(tank_system.system())
        .with_system(effect_system.system())
        .with_system(hud_system.system())
    );
    
    builder.run();
}