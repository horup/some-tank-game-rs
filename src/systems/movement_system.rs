use bevy::prelude::*;

use crate::components::{Player, Thrust};

pub fn movement_system(query:Query<(&mut Thrust, &mut Transform)>, time:Res<Time>) {
    query.for_each_mut(|(thrust, mut transform)| {
        //transform.translation.x += thrust.x * time.delta_seconds();
        //transform.translation.y += thrust.y * time.delta_seconds();
    });
}