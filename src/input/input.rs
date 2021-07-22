use std::f32::consts::PI;

use bevy::{ prelude::*};
use crate::{Autopilot, NewGameEvent, Player, Tank, Turret, WaypointEvent, mouse::Mouse};

pub fn input_system(mouse_button_input:Res<Input<MouseButton>>, 
    mouse:Res<Mouse>, keyboard_input:Res<Input<KeyCode>>, 
    mut new_game:EventWriter<NewGameEvent>, 
    mut player:Query<(&Player, &mut Tank, &mut Autopilot, &Children, &Transform)>, 
    turrets:Query<&mut Turret>,
    mut waypoint_event_writer:EventWriter<WaypointEvent>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_player, mut tank, mut autopilot, children, transform)) = player.single_mut() {
        autopilot_subsystem(&mut tank, &mut autopilot, &mouse, &mouse_button_input, &transform, &mut waypoint_event_writer);
        keyboard_subsystem(&mut tank, keyboard_input, &mut autopilot, &mut waypoint_event_writer);
        turret_subsystem(children, turrets, mouse_button_input, &mouse);
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

fn autopilot_subsystem(tank:&mut Tank, autopilot:&mut Autopilot, mouse:&Res<Mouse>, mouse_button_input: &Res<Input<MouseButton>>, transform:&Transform, waypoint_event_writer:&mut EventWriter<WaypointEvent>) {
    if autopilot.planning == false {
        if let Some(front) =  autopilot.waypoints.front() {
            // autopilot has points it needs to follow
            let goal_radius = 0.5;
            let p = transform.translation;
            let loc = front.location;
            let rot = transform.rotation;
            let f = rot * Vec3::new(1.0, 0.0, 0.0);
            let s = [rot *Vec3::new(0.0, -1.0, 0.0), rot *Vec3::new(0.0, 1.0, 0.0)];

            if p.distance(loc) <= goal_radius {
                if let Some(w) = autopilot.waypoints.pop_front() {
                    waypoint_event_writer.send(WaypointEvent::Removed(w));
                }
            } else {
                let v = (loc - p).normalize_or_zero();
                let angles = [s[0].angle_between(v), s[1].angle_between(v)];

                if f.angle_between(v) < PI / 4.0 {
                    let diff = angles[0] - angles[1];
                    if diff.abs() < 0.1 {
                        tank.tracks[0] = 1.0;
                        tank.tracks[1] = 1.0;
                    }
                    else if diff < 0.0 {
                        tank.tracks[0] = 0.0;
                        tank.tracks[1] = 1.0;
                    } else {
                        tank.tracks[0] = 1.0;
                        tank.tracks[1] = 0.0;
                    }
                } else {
                    if angles[0] < angles[1] {
                        tank.tracks[0] = -1.0;
                        tank.tracks[1] = 1.0;
                    } else {
                        tank.tracks[0] = 1.0;
                        tank.tracks[1] = -1.0;
                    }
                }

                
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
            let d = mp.distance(p);
            if d <= check_radius {
                autopilot.waypoints.clear();
                waypoint_event_writer.send(WaypointEvent::Clear);
                autopilot.planning = true;
            }
        }
    } else {
        // autopilot is in planning mode
        if mouse_button_input.pressed(MouseButton::Left) {
            // add points while pressed
            let p = mouse.pos_world.truncate().extend(0.0);
            if autopilot.any_within_radius(0.5, p) == false {
                let w = p.into();
                autopilot.waypoints.push_back(w);
                waypoint_event_writer.send(WaypointEvent::Added(w));
            } 

        } else {
            autopilot.planning = false;
        }
    }
}

fn keyboard_subsystem(tank: &mut Tank, keyboard_input: Res<Input<KeyCode>>, autopilot:&mut Autopilot, waypoint_event_writer:&mut EventWriter<WaypointEvent>) {
    
    
    let _v = Vec3::default();
    if autopilot.waypoints.len() == 0 {
        tank.tracks[0] = 0.0;
        tank.tracks[1] = 0.0;
    }

    let mut touched = false;

    let s = 1.0;
    if keyboard_input.pressed(KeyCode::W) {
        tank.tracks[0] = s;
        tank.tracks[1] = s;

        if keyboard_input.pressed(KeyCode::A) {
            tank.tracks[1] = 0.0;
            touched = true;
        } else if keyboard_input.pressed(KeyCode::D) {
            tank.tracks[0] = 0.0;
            touched = true;
        }
    }
    else if keyboard_input.pressed(KeyCode::S) {
        tank.tracks[0] = -s;
        tank.tracks[1] = -s;

        if keyboard_input.pressed(KeyCode::A) {
            tank.tracks[0] = 0.0;
            touched = true;
        } else if keyboard_input.pressed(KeyCode::D) {
            tank.tracks[1] = 0.0;
            touched = true;
        }
    } else {
        if keyboard_input.pressed(KeyCode::A) {
            tank.tracks[0] = s;
            tank.tracks[1] = -s;
            touched = true;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            tank.tracks[0] = -s;
            tank.tracks[1] = s;
            touched = true;
        }
    }

    if touched {
        autopilot.clear();
        waypoint_event_writer.send(WaypointEvent::Clear);
    }
}