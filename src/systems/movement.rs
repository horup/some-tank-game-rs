use std::f32::consts::PI;

use bevy::prelude::*;

use crate::components::{Player, Thrust};

pub fn movement_system(query:Query<(&mut Thrust, &mut Transform, &mut TextureAtlasSprite)>, time:Res<Time>) {
    query.for_each_mut(|(thrust, mut transform, mut sprite)| {

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
            let mut a = dir.angle_between(x_axis);
            if dir.y < 0.0 {
                a += PI;
            }
            transform.rotation = Quat::from_rotation_z(a);

            sprite.flip_y = !sprite.flip_y;
        }
    });
}