use bevy::prelude::*;

use crate::{ApplyDamageEvent, EffectType, Health, ThingBuilder, ThingType};

pub fn health_system(mut commands:Commands, mut health:Query<(Entity, &mut Health, &Transform)>, mut apply_damage_reader:EventReader<ApplyDamageEvent>) {
    for e in apply_damage_reader.iter() {
        if let Ok((entity, mut health, transform)) = health.get_mut(e.target) {
            health.amount -= e.amount;
            if health.amount <= 0.0 {
                commands.entity(entity).despawn_recursive();

                commands.spawn().insert(ThingBuilder {
                    translation:transform.translation,
                    thing_type:ThingType::Effect(EffectType::Explosion),
                    ..Default::default()
                });
            }
        }
    }
}