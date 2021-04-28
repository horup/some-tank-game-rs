use bevy::prelude::*;
use crate::{NewGameEvent, Player, Tank, Velocity, Turret};

pub fn input_system(mouse_button_input:Res<Input<MouseButton>>, keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>, mut player:Query<(&Player, &Tank, &mut Velocity, &Children)>, mut turrets:Query<(&mut Turret)>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_player, _, mut thrust, children)) = player.single_mut() {
        
        // tank movement input
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
        thrust.velocity = v;

        // turret input
        for e in children.iter() {
            if let Ok(mut turret) = turrets.get_component_mut::<Turret>(*e) {
                turret.trigger = mouse_button_input.pressed(MouseButton::Left);
            }
        }
    }
}