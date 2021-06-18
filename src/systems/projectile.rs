use bevy::prelude::*;
use rand::random;


use crate::{ApplyDamageEvent, EffectType, Owner, PlayAudioEvent, ProjectileHitEvent, ThingBuilder, ThingType};

pub fn projectile_system(mut play_audio:EventWriter<PlayAudioEvent>, mut commands:Commands, mut projectile_hit_events:EventReader<ProjectileHitEvent>, owners:Query<&Owner>, mut apply_damage_writer:EventWriter<ApplyDamageEvent>) {
    for hit_event in projectile_hit_events.iter() {
        if let Ok(owner) = owners.get_component::<Owner>(hit_event.projectile) {
            if owner.owner != hit_event.target {
                let mut projectile = commands.entity(hit_event.projectile);
                projectile.despawn_recursive();
                apply_damage_writer.send(ApplyDamageEvent {
                    amount:100.0,
                    target:hit_event.target
                });
                
                let mut e = commands.spawn();

                play_audio.send(format!("sfx/boom_{}.ogg", 1 + random::<u8>() % 3).into());
                e.insert(ThingBuilder {
                    translation:hit_event.location,
                    thing_type:ThingType::Effect(EffectType::BulletHit),
                    ..Default::default()
                });
            }
        }
    }
}