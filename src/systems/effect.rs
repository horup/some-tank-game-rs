use bevy::prelude::*;

use crate::Effect;


pub fn effect_system(mut commands:Commands, mut query:Query<(Entity, &mut Effect, &mut Transform)>, time:Res<Time>) {
    query.for_each_mut(|(e, mut effect, mut transform)| {
        let change = time.delta_seconds();
        effect.timer -= change;
        if effect.grow != 0.0 {
            transform.scale = transform.scale + (transform.scale * change);
        }
        if effect.timer <= 0.0 {
            commands.entity(e).despawn_recursive();
        }
    });
}