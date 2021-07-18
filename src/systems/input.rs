use bevy::{ prelude::*};
use crate::{Autopilot, NewGameEvent, Player, Tank, Turret, mouse::Mouse};

pub fn input_system(mouse_button_input:Res<Input<MouseButton>>, mouse:Res<Mouse>, keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>, mut player:Query<(&Player, &mut Tank, &mut Autopilot, &Children, &Transform)>, mut turrets:Query<&mut Turret>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_player, mut tank, mut autopilot, children, transform)) = player.single_mut() {
        if autopilot_subsystem(&mut tank, &mut autopilot, &mouse, &mouse_button_input, &transform) == false {
            keyboard_subsystem(&mut tank, keyboard_input);
            turret_subsystem(children, turrets, mouse_button_input, &mouse);
        }
    }
}

fn turret_subsystem(children: &Children, mut turrets: Query<&mut Turret>, mouse_button_input: Res<Input<MouseButton>>, mouse: &Res<Mouse>) {
    for e in children.iter() {
        if let Ok(mut turret) = turrets.get_component_mut::<Turret>(*e) {
            turret.trigger = mouse_button_input.pressed(MouseButton::Left);
           // turret.trigger = true;
            turret.target = mouse.pos_world.truncate().extend(0.0);
        }
    }
}

fn autopilot_subsystem(tank:&mut Tank, autopilot:&mut Autopilot, mouse:&Res<Mouse>, mouse_button_input: &Res<Input<MouseButton>>, transform:&Transform) -> bool {
    if autopilot.planning == false {
        
        if let Some(front) =  autopilot.waypoints.front() {
            // autopilot has points it needs to follow
            let goal_radius = 0.5;
            let p = transform.translation;
            let loc = front.location;
            let rot = transform.rotation;
            let f = Vec3::new(1.0, 0.0, 0.0);
            let f  = rot * f;

            if p.distance(loc) <= goal_radius {
                info!("reached {:?}", front.location);
                autopilot.waypoints.pop_front();
            } else {
                let v = (loc - p).normalize_or_zero();
                let a = f.angle_between(v);
                info!("{}", a);
            }
        } else {
            // no more points, stop tracks! 
            tank.tracks[0] = 0.0;
            tank.tracks[1] = 0.0;
        }

        if mouse_button_input.just_pressed(MouseButton::Left) {
            let check_radius = 0.5;
            let mp = mouse.pos_world.truncate();
            let p = transform.translation.truncate();
            if mp.distance(p) <= check_radius {
                autopilot.waypoints.clear();
                autopilot.planning = true;
            }
        }
    } else {
        // autopilot is in planning mode
        if mouse_button_input.pressed(MouseButton::Left) {
            // add points while pressed
            let p = mouse.pos_world.truncate().extend(0.0);
            autopilot.waypoints.push_back(p.into());
            info!("{}", mouse.pos_world);
        } else {
            autopilot.planning = false;
            info!("planning false!")
        }
    }

    false
}

fn keyboard_subsystem(tank: &mut Tank, keyboard_input: Res<Input<KeyCode>>) {
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
}