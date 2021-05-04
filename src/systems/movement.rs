use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::{RigidBodySet}};

use crate::{Thrust};

type Thrustable<'a> = (&'a Thrust, &'a RigidBodyHandleComponent);

pub fn movement_system(mut query:Query<Thrustable>, mut rigid_body_set:ResMut<RigidBodySet>, time:Res<Time>) {
    query.for_each_mut(|(thrust, rigid_body)| {
        let rigid_body = rigid_body_set.get_mut(rigid_body.handle()).unwrap();
        let v = thrust.impulse * time.delta_seconds();
        rigid_body.apply_impulse([v.x, v.y].into(), true);
    });
}