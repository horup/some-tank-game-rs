use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::{RigidBodySet}};

use crate::{Drag, Thrust};

type Thrustable<'a> = (&'a Thrust, &'a RigidBodyHandleComponent);
type Dragable<'a> = (&'a Drag, &'a mut RigidBodyHandleComponent);

pub fn movement_system(mut thrustable:Query<Thrustable>, mut rigid_body_set:ResMut<RigidBodySet>, time:Res<Time>) {
    /*thrustable.for_each_mut(|(thrust, rigid_body)| {
        let rigid_body = rigid_body_set.get_mut(rigid_body.handle()).unwrap();
        let v = thrust.impulse * time.delta_seconds();
        rigid_body.apply_impulse([v.x, v.y].into(), true);
    });*/
}
