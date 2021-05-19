use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::{dynamics::{RigidBodySet}, geometry::ColliderSet}};

use crate::{Drag};

type Dragable<'a> = (&'a Drag, &'a mut RigidBodyHandleComponent);

pub fn drag_system( mut dragable:Query<Dragable>, mut rigid_body_set:ResMut<RigidBodySet>, colliders:ResMut<ColliderSet>, time:Res<Time>) {
    dragable.for_each_mut(|(drag, rigid_body)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {
          /*  let rotation = rigid_body.position().rotation;
            let dir:Vec2 = [rotation.re, rotation.im].into();

            let front = dir.normalize(); 
            let side =  dir.normalize().perp();

            let v:Vec2 = [rigid_body.linvel().x, rigid_body.linvel().y].into();

            let v_front = v * front;

            let mut drag_front = v_front * time.delta_seconds() * drag.front;

            if front.x > 0.0 {
                drag_front = -drag_front;
            }

            println!("{:?}", drag_front);

            rigid_body.apply_force([drag_front.x, drag_front.y].into(), true);*/


            let v:Vec2 = [rigid_body.linvel().x, rigid_body.linvel().y].into();
            let drag = 0.5 * -v * v.length_squared() * time.delta_seconds() * 100000.0;
            //rigid_body.apply_force([drag.x, drag.y].into(), true);

            //rigid_body.apply_force([drag_front.x, drag_front.y].into(), true);


/*            let drag_front = -front * v_front * drag.front;
            let drag_side = -side * v_side * drag.side;


           // println!("{:?}", drag_side);
    
            //arigid_body.apply_force([drag_front.x, drag_front.y].into(), true);
            //rigid_body.apply_force([drag_side.x, drag_side.y].into(), true);

            println!("{:?}, {:?}", drag_front, drag_side);*/

        }
    });
}
