use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::{dynamics::{RigidBodySet}, geometry::ColliderSet}};

use crate::{Drag};

type Dragable<'a> = (&'a Drag, &'a mut RigidBodyHandleComponent);
fn project(a:Vec2, b:Vec2) -> Vec2 {
    let a1 = a.dot(b) / b.dot(b) * b;
    return a1;
}

pub fn drag_system( mut dragable:Query<Dragable>, mut rigid_body_set:ResMut<RigidBodySet>, colliders:ResMut<ColliderSet>, time:Res<Time>) {
    dragable.for_each_mut(|(drag, rigid_body)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {

            let rotation = rigid_body.position().rotation;
            let front:Vec2 = [rotation.re, rotation.im].into();
            let side = front.perp();

            let v:Vec2 = [rigid_body.linvel().x, rigid_body.linvel().y].into();

            let drag_front = -project(v, front) * time.delta_seconds() * drag.front;
            let drag_side = -project(v, side) * time.delta_seconds() * drag.side;

            rigid_body.apply_impulse([drag_front.x, drag_front.y].into(), true);
            rigid_body.apply_impulse([drag_side.x, drag_side.y].into(), true);
        }
    });
}
