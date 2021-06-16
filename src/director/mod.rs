use bevy::prelude::*;
use crate::{AppState, Bot, Console, GameState, Hud, PlayAudioEvent, Player};


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
    pub quick:bool,
    pub level:i32,
    pub levels:i32, 
    pub timer:f32,
    pub state:DirectorState
}

impl Director {
    pub fn transition(&mut self, new_state:DirectorState, secs:f32) {
        self.timer = secs;
        if self.quick {
            self.timer = 0.0;
        }
        self.state = new_state;
    }
}

impl Default for Director {
    fn default() -> Self {
        Self {
            quick:false,
            level:1,
            levels:4,
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

    if director.timer > 0.0 {
        director.timer -= time.delta_seconds();
        return;
    }
    director.timer = 0.0;

    let is_player_alive = players.iter().len() > 0;
    let some_enemies_left = bots.iter().len() > 0;

    match director.state {
        DirectorState::StartLoadLevel => {
            hud.start_default_fade();
            director.transition(DirectorState::LoadLevel, 0.5);
        }
        DirectorState::LoadLevel => {
            hud.clear_texts();
            console.load_map(director.level.to_string().as_str());
            director.transition(DirectorState::GetReady, 0.0);
        },
        DirectorState::GetReady => {
            play_audio.send("sfx/get_ready.mp3".into());
            hud.center_text = "Get Ready!!!".into();
            hud.top_left_text = "Level ".to_string() + director.level.to_string().as_str() + " of " + &director.levels.to_string();
            director.transition(DirectorState::Go, 1.5);
        },
        DirectorState::Go => {
            play_audio.send("sfx/go.mp3".into());
            hud.center_text = "Go!!!".into();
            let _ = game_state.overwrite_set(GameState::Running);

            director.transition(DirectorState::InProgress, 1.0);
        },
        DirectorState::InProgress => {
            if is_player_alive == false {
                director.transition(DirectorState::Died, 1.0);
            } else if some_enemies_left == false {
                if director.level == director.levels {
                    director.transition(DirectorState::WonGame, 1.0);
                } else {
                    director.transition(DirectorState::WonLevel, 1.0);
                }
            }
            hud.center_text = "".into();
        },
        DirectorState::Died => {
            play_audio.send("sfx/too_bad.mp3".into());
            hud.center_text = "You died! Restarting level...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.transition(DirectorState::StartLoadLevel, 1.0);
        },
        DirectorState::WonLevel => {
            play_audio.send("sfx/great.mp3".into());
            hud.center_text = "All Enemies are dead!\nStarting next level...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.level += 1;
            director.transition(DirectorState::StartLoadLevel, 1.0);
        },
        DirectorState::WonGame => {
            play_audio.send("sfx/won.mp3".into());
            hud.center_text = "You Won the Game!\nCongratulations!\nClick to restart the game...".into();
            let _ = game_state.overwrite_set(GameState::Paused);
            director.transition(DirectorState::AwaitRestartGameInput, 0.5);
        },
        DirectorState::AwaitRestartGameInput => {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                // clear director and start load level
                *director = Director::default();
                director.transition(DirectorState::StartLoadLevel, 0.0);
            }
        },
    }
    
}

pub struct DirectorPlugin;

impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(Director::default())
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update.system()));
    }
}