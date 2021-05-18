use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::Tank;

pub fn tank_system(tank:Query<(&Tank, &RigidBodyHandleComponent)>, mut rigid_body_set:ResMut<RigidBodySet>, time:Res<Time>) {
    tank.for_each_mut(|(tank, rigid_body)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {
            println!("{:?}", tank.tracks);
        }
    });
}