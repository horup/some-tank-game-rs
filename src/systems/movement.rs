use bevy::prelude::*;

use crate::components::{Player, Thrust};

pub fn movement_system(query:Query<(&mut Thrust, &mut Transform)>, time:Res<Time>) {
    query.for_each_mut(|(thrust, mut transform)| {

        let mut v = thrust.force;

        if thrust.constrained {
            if thrust.force.x.abs() > 0.0 {
                v.y = 0.0;
            } else if thrust.force.y.abs() > 0.0 {
                v.x = 0.0;
            }
        }

        if v.length() > 0.0 {
            transform.translation += v * time.delta_seconds();
            let dir = v.normalize();
            let x_axis = Vec3::new(1.0, 0.0, 0.0);
            let a = dir.angle_between(x_axis);
            transform.rotation = Quat::from_rotation_z(a);
        }
    });
}