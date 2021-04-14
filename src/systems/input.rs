use bevy::prelude::*;
use crate::NewGameEvent;

pub fn input_system(keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>) {
    if keyboard_input.pressed(KeyCode::A) {
    }

    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }
}