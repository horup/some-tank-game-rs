use std::{collections::HashMap};

use bevy::{diagnostic::{LogDiagnosticsPlugin}};

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

mod director;
pub use director::*;

mod hud;
pub use hud::*;

mod tiled;

mod console;
pub use console::*;

mod map_loader;
pub use map_loader::*;

mod splash;
pub use splash::*;

mod delay_state;
pub use delay_state::*;

mod persister;
pub use persister::*;

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

fn state_input(mut console:ResMut<Console>, input:Res<Input<KeyCode>>) {
    let mut save = false;
    if input.pressed(KeyCode::LShift) || input.pressed(KeyCode::RShift) {
        save = true;
    }

    let mut map:HashMap<KeyCode, u8> = HashMap::default();
    map.insert(KeyCode::F1, 0);
    map.insert(KeyCode::F2, 1);
    map.insert(KeyCode::F3, 2);
    map.insert(KeyCode::F4, 3);
    map.insert(KeyCode::F5, 4);
    map.insert(KeyCode::F6, 5);
    map.insert(KeyCode::F7, 6);
    map.insert(KeyCode::F8, 7);

    for (key_code, index) in map {
        if input.just_pressed(key_code) {
            if save {
                console.save_state(index);
            } else {
                console.load_state(index);
            }
            return;
        }
    }

}

fn startup_system(mut commands:Commands, mut rapier:ResMut<RapierConfiguration>, mut app_state:ResMut<State<AppState>>, asset_server:Res<AssetServer>, audio:Res<Audio>, audio_source:Res<Assets<AudioSource>>) {
    // cameras
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GameCamera::default());

    rapier.gravity.x = 0.0;
    rapier.gravity.y = 0.0;
    rapier.time_dependent_number_of_timesteps = true;

    app_state.push(AppState::Splash.into()).unwrap();
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
    builder.add_state(AppState::default());
    builder.add_state(GameState::default());
    builder.add_system(debug.system());
    builder.add_system(state_input.system());

    // add plugins
    builder.add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPluginCustom)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(crate::tiled::TiledPlugin)
    .add_plugin(TilemapPlugin::default())
    .add_plugin(SpriteBuilderPlugin::default())
    .add_plugin(EventsPlugin::default())
    .add_plugin(DirectorPlugin)
    .add_plugin(HudPlugin)
    .add_plugin(ConsolePlugin)
    .add_plugin(MapLoaderPlugin)
    .add_plugin(SplashPlugin)
    .add_plugin(DelayPlugin::<AppState>::default())
    .add_plugin(PersisterPlugin);

    
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