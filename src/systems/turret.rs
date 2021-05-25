
use bevy::prelude::*;

use crate::{ThingBuilder, Turret};


pub fn turret_system(mut commands:Commands, turrets:Query<(Entity, &mut Turret, &Parent)>, mut transforms:Query<(&mut Transform,)>, time:Res<Time>) {
    turrets.for_each_mut(|(turret_entity, turret, parent_entity), | {
        let mut parent_translation = Vec3::default();
        let mut parent_rotation = Quat::default();
        if let Ok(parent_transform) = transforms.get_component::<Transform>(parent_entity.0) {
            parent_translation = parent_transform.translation;
            parent_rotation = parent_transform.rotation;
        }
        if let Ok(mut turret_transform) = transforms.get_component_mut::<Transform>(turret_entity) {
            let target = turret.target;
            let pos = parent_translation;
            let v = target - pos;

            if v.length() > 0.0 {
                let b = v.normalize();
                let a = Vec3::new(1.0, 0.0, 0.0);
                let rot_global = Quat::from_rotation_arc(a, b);
                let a = parent_rotation.mul_vec3(a);
                let rot_relative = Quat::from_rotation_arc(a, b);

                turret_transform.rotation = rot_relative;

                let mut turret = turret;
                turret.cooldown -= time.delta_seconds();
                if turret.cooldown <= 0.0 {
                    turret.cooldown = 0.0;
                }

                if turret.cooldown == 0.0 && turret.trigger {
                    turret.cooldown = 1.0;
                    let mut e = commands.spawn();
                    let v = Vec3::new(1.0, 0.0, 0.0) * 0.5;
                    let v =  rot_global * v;
                    e.insert(ThingBuilder {
                        translation:parent_translation + v,
                        rotation:rot_global,
                        thing_type:crate::ThingType::Bullet,
                        owner:Some(parent_entity.0)
                    });

                }
            }
        }
    });

}