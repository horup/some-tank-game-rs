use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::{dynamics::{RigidBodySet}, geometry::ColliderSet}};

use crate::{Drag};

type Dragable<'a> = (&'a Drag, &'a mut RigidBodyHandleComponent);

pub fn drag_system( mut dragable:Query<Dragable>, mut rigid_body_set:ResMut<RigidBodySet>, colliders:ResMut<ColliderSet>, time:Res<Time>) {
    dragable.for_each_mut(|(drag, rigid_body)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {
            let rotation = rigid_body.position().rotation;
            let dir:Vec2 = [rotation.re, rotation.im].into();

            //println!("{:?}", dir);
            let drag_front = drag.drag_front.truncate();
            let drag_side = drag.drag_side.truncate();
    
            let front = Vec2::new(1.0, 0.0); 
            
            let side = Vec2::new(0.0, 1.0);

            let v:Vec2 = [rigid_body.linvel().x, rigid_body.linvel().y].into();
            let v_front = v * front;
            let v_side = v * side;
            
            //println!("{:?}", v_front);

            let drag_front = -v_front.normalize_or_zero() * drag_front * v_front.length_squared();
            let drag_side = -v_side.normalize_or_zero() * drag_side * v_side.length_squared();
           // println!("{:?}", drag_side);
    
            rigid_body.apply_force([drag_front.x, drag_front.y].into(), true);
            rigid_body.apply_force([drag_side.x, drag_side.y].into(), true);
        }
    });
}
