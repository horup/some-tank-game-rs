use bevy::prelude::*;

use crate::Velocity;

pub fn movement_system(mut query:Query<(&mut Transform, &Velocity)>, time:Res<Time>) {

    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation += velocity.velocity * time.delta_seconds();
    });
}