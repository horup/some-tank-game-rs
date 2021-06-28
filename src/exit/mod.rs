use bevy::prelude::*;

use crate::{AppState, GameState, Hud};

#[derive(Default)]
pub struct Exit {
    pub paused_the_game:bool
}
pub struct ExitPlugin;


fn input(input:Res<Input<KeyCode>>, mut hud:ResMut<Hud>, mut app_state:ResMut<State<AppState>>, mut game_state:ResMut<State<GameState>>, mut exit:ResMut<Exit>) {
    let escaped = input.just_pressed(KeyCode::Escape);
    let y = input.just_pressed(KeyCode::Y);
    if app_state.current() != &AppState::ShowExit {
        if escaped {
            hud.push();
            hud.clear();
            hud.center_text = "Press Y to exit the game\nor Escape to return".into();
            if app_state.current() != &AppState::Splash {
                hud.background = Color::rgba(0.0, 0.0, 0.0, 0.75);
            }

            if game_state.current() == &GameState::Running {
                exit.paused_the_game = true;
                game_state.push(GameState::Paused).unwrap();
            } else {
                exit.paused_the_game = false;
            }

            app_state.push(AppState::ShowExit).unwrap();
        }
        
    } else {
        if escaped {
            if exit.paused_the_game == true {
                game_state.pop().unwrap();
            }
            
            hud.pop();
            app_state.pop().unwrap();
        } else if y {
            std::process::exit(0);
        }
    }
}

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Exit::default());
        app.add_system_to_stage(CoreStage::First,input.system());
    }
}