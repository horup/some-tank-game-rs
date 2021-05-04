use std::f32::consts::PI;

use bevy::{prelude::*, sprite::collide_aabb};

use crate::{Body, Tilemap, components::{Velocity}};

pub fn physics_system(query:Query<(&mut Velocity, &mut Transform, &Body)>, time:Res<Time>, tilemap_query:Query<(&Tilemap)>) {

}