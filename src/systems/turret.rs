
use bevy::prelude::*;

use crate::{Factory, Turret, resources::Textures};


pub fn turret_system() {
    
}
/*
pub fn turret_system(mut commands:Commands, turrets:Query<(Entity, &mut Turret, &Parent)>, mut transforms:Query<(&mut Transform,)>, mut velocities:Query<(&Velocity)>, time:Res<Time>, textures:Res<Textures>)
{
    turrets.for_each_mut(|(turret_entity, turret, tank_parent), | {
        let mut parent_translation = Vec3::default();
        let mut parent_rotation = Quat::default();
        if let Ok(transform) = transforms.get_component::<Transform>(tank_parent.0) {
            parent_translation = transform.translation;
            parent_rotation = transform.rotation;
        }

        if let Ok(mut turret_transform) = transforms.get_component_mut::<Transform>(turret_entity) {
           
            let def = Velocity::default();
            let velocity = velocities.get_component::<Velocity>(tank_parent.0).unwrap_or(&def);
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

                    let mut factory = Factory::new(&mut commands, &textures);
                    factory.spawn_projectile(parent_translation, rot_global, &velocity,tank_parent.0);
                }
            }
        }

    });
}*/