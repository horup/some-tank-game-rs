use bevy::prelude::*;
use crate::{NewGameEvent, Player, Thrust};

pub fn input_system(keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>, mut player:Query<(&Player, &mut Thrust)>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }



    if let Ok((_player, mut thrust)) = player.single_mut() {
        let speed = 2.0;
        let mut v = Vec3::default();
        if keyboard_input.pressed(KeyCode::W) {
            v.y = 1.0;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            v.y = -1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            v.x = -1.0;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            v.x = 1.0;
        }

        if v.length() > 0.0 {
            v = v.normalize() * speed;
        }

        thrust.force = v;

    }
}