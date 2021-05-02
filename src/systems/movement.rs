use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb};

use crate::components::{Velocity};

pub fn movement_system(query:Query<(&mut Velocity, &mut Transform, &mut TextureAtlasSprite)>, time:Res<Time>) {
    query.for_each_mut(|(thrust, mut transform, mut sprite)| {

        let mut v = thrust.velocity;

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