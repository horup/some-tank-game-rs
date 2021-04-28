
use bevy::prelude::*;

use crate::{Factory, Turret, resources::Textures};


pub fn turret_system(mut commands:Commands, turrets:Query<(Entity, &mut Turret, &Parent)>, mut transforms:Query<(&mut Transform,)>, time:Res<Time>, textures:Res<Textures>)
{
    turrets.for_each_mut(|(turret_entity, turret, tank_parent), | {
        let mut parent_translation = Vec3::default();
        let mut parent_rotation = Quat::default();
        if let Ok(transform) = transforms.get_component::<Transform>(tank_parent.0) {
            parent_translation = transform.translation;
            parent_rotation = transform.rotation;
        }

        if let Ok(mut turret_transform) = transforms.get_component_mut::<Transform>(turret_entity) {
           
            let target = turret.target;
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
            let mut turret = turret;
            turret.cooldown -= time.delta_seconds();
            if turret.cooldown <= 0.0 {
                turret.cooldown = 0.0;
            }

            if turret.cooldown == 0.0 && turret.trigger {
                turret.cooldown = 1.0;

                let mut factory = Factory::new(&mut commands, &textures);
                factory.spawn_projectile(parent_translation, parent_rotation, tank_parent.0);
            }
        }
    });
}