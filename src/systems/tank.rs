use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::Tank;

pub fn tank_system(tank:Query<(&Tank, &RigidBodyHandleComponent)>, mut rigid_body_set:ResMut<RigidBodySet>, time:Res<Time>) {
    tank.for_each_mut(|(tank, rigid_body)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {
            let dir:Vec2 = [rigid_body.position().rotation.re, rigid_body.position().rotation.im].into();
            let diff = tank.tracks[0] - tank.tracks[1];
            let force = (dir * tank.tracks[0] + dir * tank.tracks[1]) * time.delta_seconds() * 100.0;
            
            rigid_body.set_angvel(diff, true);
            rigid_body.apply_force([force.x, force.y].into(), true);
        }
    });
}