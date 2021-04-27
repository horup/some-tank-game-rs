
use bevy::prelude::*;

use crate::{Turret};


pub fn turret_system(mut commands:Commands, turrets:Query<(Entity, &mut Turret, &Parent)>, mut transforms:Query<(&mut Transform,)>, time:Res<Time>)
{
    turrets.for_each_mut(|turret| {
        let mut parent_translation = Vec3::default();
        let mut parent_rotation = Quat::default();
        if let Ok(transform) = transforms.get_component::<Transform>(turret.2.0) {
            parent_translation = transform.translation;
            parent_rotation = transform.rotation;
        }

        if let Ok(mut turret_transform) = transforms.get_component_mut::<Transform>(turret.0) {
           
            let target = turret.1.target;
            let pos = parent_translation;
            let v = target - pos;

            if v.length() > 0.0 {
                let b = v.normalize();
                let a = Vec3::new(1.0, 0.0, 0.0);
                let a = parent_rotation.mul_vec3(a);
                let rot = Quat::from_rotation_arc(a, b);

                turret_transform.rotation = rot;
            }
        }

        {
            let mut turret = turret.1;
            turret.cooldown -= time.delta_seconds();
            if turret.cooldown <= 0.0 {
                turret.cooldown = 0.0;
            }

            if turret.cooldown == 0.0 && turret.trigger {
                turret.cooldown = 1.0;
                Fac
            }
        }
        
    });
}