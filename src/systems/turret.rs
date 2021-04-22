use bevy::prelude::*;

use crate::{Thrust, Turret};

pub fn turret_system(turrets:Query<(&Turret, &mut Transform, &Parent)>, transforms:Query<(Entity, &GlobalTransform, &Thrust)>) {
    turrets.for_each_mut(|(turret, mut transform, parent)| {
        let parent = parent.0;
        if let Ok(parent_transform) = transforms.get_component::<GlobalTransform>(parent) {
            let facing = parent_transform.rotation.
            
            //
            let target = turret.target;
            let pos = parent_transform.translation;
            let v = target - pos;
            if v.length() > 0.0 {
                let v = v.normalize();

                let rot = Quat::from_rotation_arc(, to)
                //transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), )

                transform.rotation = parent_transform.rotation;
            }
        }
    });
}