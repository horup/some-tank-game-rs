use bevy::{ prelude::*};
use crate::{NewGameEvent, Player, Tank, Turret, resources::Mouse};

pub fn input_system(mouse_button_input:Res<Input<MouseButton>>, mouse:Res<Mouse>, keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>, mut player:Query<(&Player, &mut Tank, &Children)>, mut turrets:Query<&mut Turret>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_player, mut tank, children)) = player.single_mut() {
        
        // tank movement input
        let _v = Vec3::default();
        tank.tracks[0] = 0.0;
        tank.tracks[1] = 0.0;
        let s = 1.0;
        if keyboard_input.pressed(KeyCode::W) {
            tank.tracks[0] = s;
            tank.tracks[1] = s;

            if keyboard_input.pressed(KeyCode::A) {
                tank.tracks[1] = 0.0;
            } else if keyboard_input.pressed(KeyCode::D) {
                tank.tracks[0] = 0.0;
            }
        }
        else if keyboard_input.pressed(KeyCode::S) {
            tank.tracks[0] = -s;
            tank.tracks[1] = -s;

            if keyboard_input.pressed(KeyCode::A) {
                tank.tracks[0] = 0.0;
            } else if keyboard_input.pressed(KeyCode::D) {
                tank.tracks[1] = 0.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::A) {
                tank.tracks[0] = s;
                tank.tracks[1] = -s;
            }
            else if keyboard_input.pressed(KeyCode::D) {
                tank.tracks[0] = -s;
                tank.tracks[1] = s;
            }
        }

        // turret input
        for e in children.iter() {
            if let Ok(mut turret) = turrets.get_component_mut::<Turret>(*e) {
                turret.trigger = mouse_button_input.pressed(MouseButton::Left);
               // turret.trigger = true;
                turret.target = mouse.pos_world.truncate().extend(0.0);
            }
        }
    }
}