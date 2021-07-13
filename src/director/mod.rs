
use bevy::{prelude::*};
use crate::{AppState, AssetCache, Bot, Config, Console, GameState, Hud, Json, PlayAudioEvent, Player};

mod levels;
pub use levels::*;

enum DirectorState {
    StartLoadLevel,
    LoadLevel,
    GetReady,
    InProgress,
    Go,
    Died,
    WonLevel,
    WonGame,
    AwaitRestartGameInput
}

struct Director {
    pub ready:bool,
    pub quick:bool,
    pub current_level:u32,
    pub levels:Levels, 
    pub timer:f32,
    pub state:DirectorState
}

impl Director {
    pub fn transition(&mut self, new_state:DirectorState, secs:f32) {
        self.timer = secs;
        if self.quick {
            self.timer = 0.25;
        }
        self.state = new_state;
    }

    pub fn reset(&mut self) {
        self.current_level = 1;
    }
}

impl Default for Director {
    fn default() -> Self {
        Self {
            ready:false,
            quick:false,
            current_level:1,
            levels:Levels::default(),
            timer:0.0,
            state:DirectorState::LoadLevel
        }
    }
}

fn update(
    mut director:ResMut<Director>, 
    players:Query<&Player>, 
    bots:Query<&Bot>, 
    time:Res<Time>, 
    mut game_state:ResMut<State<GameState>>,
    mut console:ResMut<Console>,
    mut hud:ResMut<Hud>,
    mouse_button_input:Res<Input<MouseButton>>,
    mut play_audio:EventWriter<PlayAudioEvent>) {

    if director.ready == false {
        return;
    }

    if director.timer > 0.0 {
        director.timer -= time.delta_seconds();
        return;
    }
    director.timer = 0.0;

    let is_player_alive = players.iter().len() > 0;
    let some_enemies_left = bots.iter().len() > 0;


    match director.state {
        DirectorState::StartLoadLevel => {
            if director.quick == false {
                hud.start_default_fade();
            }
            
            director.transition(DirectorState::LoadLevel, 0.5);
        }
        DirectorState::LoadLevel => {
            hud.clear_texts();
            console.load_map(&director.levels.get_map(director.current_level));
            director.transition(DirectorState::GetReady, 0.0);
        },
        DirectorState::GetReady => {
            play_audio.send("sfx/get_ready.ogg".into());
            hud.center_text = "Get Ready!!!".into();
            hud.top_left_text = "Level ".to_string() + director.current_level.to_string().as_str() + " of " + &director.levels.count().to_string();
            director.transition(DirectorState::Go, 1.5);
        },
        DirectorState::Go => {
            play_audio.send("sfx/go.ogg".into());
            hud.center_text = "Go!!!".into();
            let _ = game_state.overwrite_set(GameState::Running);

            director.transition(DirectorState::InProgress, 1.0);
            play_audio.send(PlayAudioEvent::new("music/Zander Noriega - Fight Them Until We Cant.ogg").with_music(true));
        },
        DirectorState::InProgress => {
            if is_player_alive == false {
                director.transition(DirectorState::Died, 1.0);
            } else if some_enemies_left == false {
                if director.current_level == director.levels.count() {
                    director.transition(DirectorState::WonGame, 1.0);
                } else {
                    director.transition(DirectorState::WonLevel, 1.0);
                }
            }
            hud.center_text = "".into();
        },
        DirectorState::Died => {
            play_audio.send(PlayAudioEvent::new("").with_music(true));
            play_audio.send("sfx/too_bad.ogg".into());
            hud.center_text = "You died! Restarting level...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.transition(DirectorState::StartLoadLevel, 1.0);
        },
        DirectorState::WonLevel => {
            play_audio.send(PlayAudioEvent::new("").with_music(true));
            play_audio.send("sfx/great.ogg".into());
            hud.center_text = "All Enemies are dead!\nStarting next level...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.current_level += 1;
            director.transition(DirectorState::StartLoadLevel, 1.0);
        },
        DirectorState::WonGame => {
            play_audio.send(PlayAudioEvent::new("").with_music(true));
            play_audio.send("sfx/won.ogg".into());
            hud.center_text = "You Won the Game!\nCongratulations!\nClick to restart the game...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.transition(DirectorState::AwaitRestartGameInput, 0.5);
        },
        DirectorState::AwaitRestartGameInput => {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                // reset game
                director.reset();
                director.transition(DirectorState::StartLoadLevel, 0.0);
            }
        },
    }
    
}

fn startup(mut director:ResMut<Director>, config:Res<Config>) {
    director.quick = config.quick();
}

fn load_director(mut director:ResMut<Director>, asset_server:Res<AssetServer>, json:Res<Assets<Json>>, mut asset_cache:ResMut<AssetCache>) {
    
    if director.ready == false {
        let handle:Handle<Json> = asset_server.get_handle("levels.json");
        if asset_cache.contains(&handle) == false {
            let handle:Handle<Json> = asset_server.load("levels.json");
            asset_cache.track(&handle);
        } else if asset_cache.is_loaded(&handle) {
            let json = json.get(handle).unwrap();
            if let Some(maps) = json.as_object().and_then(|o|o.get("maps").and_then(|v| v.as_array())) {
                director.levels.maps.clear();
                for map in maps {
                    if let Some(name) = map.as_str() {
                        director.levels.maps.push(name.into());
                    }
                }
            }

            director.ready = true;
        }
    }

}

pub struct DirectorPlugin;

impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(Director::default())
        .add_startup_system(startup.system())
        .add_system(load_director.system())
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update.system()));
    }
}