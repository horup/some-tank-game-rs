use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::{RigidBodySet}};

use crate::{Thrust};

type Thrustable<'a> = (&'a Thrust, &'a RigidBodyHandleComponent);

pub fn movement_system(mut query:Query<Thrustable>, mut rigid_body_set:ResMut<RigidBodySet>) {
    query.for_each_mut(|(thrust, rigid_body)| {
        let rigid_body = rigid_body_set.get_mut(rigid_body.handle()).unwrap();
        rigid_body.apply_impulse([thrust.impulse.x, thrust.impulse.y].into(), true);
        println!("lol");
    });
}