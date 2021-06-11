use bevy::prelude::*;
use crate::{AppState, Bot, Console, Hud, Player};

use bevy::core::Time;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum GameState {
    NotSet,
    Loading,
    GetReady,
    Go,
    InProgress,
    Failure,
    Success,
    YouWon
}

pub struct GameDirector {
    pub state:GameState,
    pub next_state_at:Option<(GameState, f64)>,
    pub quick:bool,
    pub level:i32,
    pub levels:i32
}

pub struct GameStateChangeEvent {
    pub from:GameState,
    pub to:GameState
}

impl GameDirector {
    pub fn transition(&mut self, next_state:GameState, in_seconds:f64, time:&Time) {
        self.next_state_at = Some((next_state, time.seconds_since_startup()  + in_seconds));
    }

    pub fn transition_asap(&mut self, next_state:GameState) {
        self.next_state_at = Some((next_state, 0.0));
    }

    pub fn clear_transition(&mut self) {
        self.next_state_at = None;
    }

    pub fn next_state(&self) -> Option<GameState> {
        Some(self.next_state_at?.0)
    }
}

impl Default for GameDirector {
    fn default() -> Self {
        Self {
            state:GameState::NotSet,
            next_state_at:Some((GameState::Loading, 0.0)),
            quick:false,
            level:1,
            levels:4
        }
    }
}

fn game_system(mouse_button_input:Res<Input<MouseButton>>, mut console:ResMut<Console>, mut game:ResMut<GameDirector>, mut game_state_change_reader:EventReader<GameStateChangeEvent>, mut hud:ResMut<Hud>, time:Res<Time>, players:Query<&Player>, bots:Query<&Bot>, mut app_state:ResMut<State<AppState>>) {
    for e in game_state_change_reader.iter() {
        match e.to {
            GameState::NotSet => {
            }
            GameState::GetReady => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "Get Ready!".into();
                hud.top_left_text = "Level ".to_string() + game.level.to_string().as_str() + " of " + &game.levels.to_string();
                console.load_map(game.level.to_string().as_str());
                game.transition(GameState::Go, 3.0, &time);
            }
            GameState::Go => {
                let _ = app_state.set(AppState::InGame);
                hud.center_text = "Go!".into();
                game.transition(GameState::InProgress, 1.0, &time);
            }
            GameState::InProgress => {
                let _ = app_state.set(AppState::InGame);
                hud.center_text = "".into();
            }
            GameState::Loading => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "Loading...".into();
                game.level = 1;
                game.transition_asap(GameState::GetReady);
            }
            GameState::Failure => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "You Died!\nRestarting level...".into();
                game.transition(GameState::GetReady, 3.0, &time);
            }
            GameState::Success => {
                let _ = app_state.set(AppState::InBetweenGames);

                if game.level < game.levels {
                    hud.center_text = "All Enemies are dead!\nStarting next level...".into();
                    game.level += 1;
                    game.transition(GameState::GetReady, 3.0, &time);
                } else {
                    hud.center_text = "You Won the Game!\nCongratulations!\nClick to restart the game...".into();
                    game.transition(GameState::YouWon, 1.0, &time);
                }
            }
            GameState::YouWon => {
            }
        }

    }

    if game.state == GameState::InProgress {
        let mut player_is_dead = true;
        players.for_each(|_| {
            player_is_dead = false;
        });

    
        if player_is_dead {
            if game.next_state() == None {
                game.transition(GameState::Failure, 3.0, &time);
            }
        } else {
            // check if no bots are left
            let bots = bots.iter().len();
            if bots == 0 {
                if game.next_state() == None {
                    game.transition(GameState::Success, 3.0, &time);
                }
            }
        }
    } else if game.state == GameState::YouWon && game.next_state() == None && mouse_button_input.just_pressed(MouseButton::Left) {
        game.transition(GameState::Loading, 0.0, &time);
    }
}

fn game_tick_system(app_state:ResMut<State<AppState>>, mut game:ResMut<GameDirector>, time:Res<Time>, mut game_state_change_writer:EventWriter<GameStateChangeEvent>) {
    if let Some((next_state, at)) = game.next_state_at {
        if at <= time.seconds_since_startup() || game.quick {
            let prev = game.state;
            game.state = next_state;
            game.clear_transition();
            game_state_change_writer.send(GameStateChangeEvent {
                from:prev,
                to:game.state
            });
        }
    }
}
 
pub struct GameDirectorPlugin;

impl Plugin for GameDirectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_event::<GameStateChangeEvent>()
        .insert_resource(GameDirector::default())
        .add_system_to_stage(CoreStage::PreUpdate,game_tick_system.system())
        .add_system(game_system.system());
    }
}