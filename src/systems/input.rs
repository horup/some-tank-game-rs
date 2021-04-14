use bevy::prelude::*;

pub fn input_system(keyboard_input:Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("lol");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("lol2");
    }
}