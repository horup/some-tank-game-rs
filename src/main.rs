use bevy::{diagnostic::{LogDiagnosticsPlugin}, prelude::*};

mod components;
use bevy_rapier2d::physics::{RapierConfiguration};
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;

mod resources;
use resources::*;

mod plugins;
pub use plugins::*;

mod game_director;
pub use game_director::*;

mod hud;
pub use hud::*;

mod tiled;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Pause,
    Running
}


fn startup_system(mut commands:Commands, mut rapier:ResMut<RapierConfiguration>) {
    // cameras
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GameCamera::default());

    rapier.gravity.x = 0.0;
    rapier.gravity.y = 0.0;
    rapier.time_dependent_number_of_timesteps = true;
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::build();
    let window = WindowDescriptor {
        title: "Blueprint 3.0".to_string(),
        width: 1024.0,
        height: 768.0,
        vsync: true,
        ..Default::default()
    };
    builder.insert_resource(window);
    builder.add_state(AppState::Pause);

    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPluginCustom)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(crate::tiled::TiledPlugin)
    .add_plugin(TilemapPlugin::default())
    .add_plugin(SpriteBuilderPlugin::default())
    .add_plugin(EventsPlugin::default())
    .add_plugin(GameDirectorPlugin)
    .add_plugin(HudPlugin);
    
    // add resources
    builder
    .insert_resource(Mouse::default())

    .insert_resource(Hud::default());

    // add events
    builder.add_event::<NewGameEvent>();
    
    

    // add startup systems
    builder
    .add_startup_system(startup_system.system());
    //.add_startup_system(load_textures_system.system());

    // add always on systems
    builder
    .add_system(camera_system.system())
    .add_system(faction_system.system());

    // add in game update systems
    builder.add_system_set_to_stage(CoreStage::Update, 
        SystemSet::on_update(AppState::Running)
        .with_system(input_system.system())
        .with_system(mouse_input_system.system())
    );
    builder
    .add_system_set(SystemSet::on_update(AppState::Running)
        .with_system(drag_system.system())
        .with_system(turret_system.system())
        .with_system(bot_system.system())
        .with_system(bot_sensor_system.system())
        .with_system(projectile_system.system().after("physics"))
        .with_system(physics_system.system().label("physics"))
        .with_system(health_system.system())
        .with_system(tank_system.system())
        .with_system(effect_system.system())
    );
    
    builder.run();
}