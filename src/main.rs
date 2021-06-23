#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

use bevy::{diagnostic::{LogDiagnosticsPlugin}, window::WindowResizeConstraints};

mod components;
use bevy_rapier2d::physics::{RapierConfiguration};
pub use components::*;

mod events;
pub use events::*;

mod systems;
use systems::*;

mod resources;
use resources::*;

mod director;
pub use director::*;

mod hud;
pub use hud::*;

mod console;
pub use console::*;

mod map_loader;
pub use map_loader::*;

mod splash;
pub use splash::*;

mod persister;
pub use persister::*;

pub use extensions::*;

mod asset_cache;
pub use asset_cache::*;

mod thing_builder;
pub use thing_builder::*;

mod rapier;
pub use rapier::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Splash,
    InGame
}

impl Default for AppState {
    fn default() -> Self {
        Self::InGame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Paused,
    Running
}

impl Default for GameState {
    fn default() -> Self {
        Self::Paused
    }
}



fn debug(mut char_input_reader:EventReader<ReceivedCharacter>, mut console:ResMut<Console>) {
    for e in char_input_reader.iter() {
        if ['1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&e.char) {
            console.load_map(e.char.to_string().as_str());
        }
    }
}

fn state_input(input:Res<Input<KeyCode>>, mut hud:ResMut<Hud>) {
    if input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if input.just_pressed(KeyCode::F1) || input.just_pressed(KeyCode::Grave) {
        hud.show_console = !hud.show_console;
    }
}

fn startup_system(mut commands:Commands, mut rapier:ResMut<RapierConfiguration>, mut app_state:ResMut<State<AppState>>) {
    // cameras
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GameCamera::default());

    rapier.gravity.x = 0.0;
    rapier.gravity.y = 0.0;
    rapier.time_dependent_number_of_timesteps = true;

    app_state.push(AppState::Splash.into()).unwrap();
}

#[derive(Default)]
struct Logger {

}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() { 
    //let config = Ini::load_from_file("config.ini").unwrap_or_default();
//let debug = config..get_from_or("debug", "enabled", "false");
 //   println!("debug={}", debug);


    let mut builder = App::build();
    let window = WindowDescriptor {
        title: "Some Tank Game!".to_string(),
        width: 1024.0,
        height: 768.0,
        vsync: true,
        resize_constraints:WindowResizeConstraints {
            min_width: 1024.0,
            min_height: 768.0,
            max_width: f32::MAX,
            max_height: f32::MAX,
        },
        ..Default::default()
    };
 
    builder.insert_resource(window);
    builder.add_state(AppState::default());
    builder.add_state(GameState::default());
    builder.add_system(debug.system());
    builder.add_system(state_input.system());

    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPluginCustom)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(TiledLoaderPlugin)
    .add_plugin(TilemapPlugin::default())
    .add_plugin(SpriteBuilderPlugin::default())
    .add_plugin(EventsPlugin::default())
    .add_plugin(DirectorPlugin)
    .add_plugin(HudPlugin)
    .add_plugin(ConsolePlugin)
    .add_plugin(MapLoaderPlugin)
    .add_plugin(SplashPlugin)
    .add_plugin(DelayPlugin::<AppState>::default())
    .add_plugin(PersisterPlugin)
    .add_plugin(AudioPlugin)
    .add_plugin(JsonLoaderPlugin)
    .add_plugin(AssetCachePlugin);

    
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
        SystemSet::on_update(AppState::InGame)
        .with_system(input_system.system())
        .with_system(mouse_input_system.system())
    );
    builder
    .add_system_set(SystemSet::on_update(GameState::Running)
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