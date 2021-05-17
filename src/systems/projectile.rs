use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{Collider, ColliderHandle, ColliderSet, ContactEvent}};

use crate::{ApplyDamageEvent, Health, Owner, Projectile, ProjectileHitEvent};

pub fn projectile_system(mut commands:Commands, mut projectile_hit_events:EventReader<ProjectileHitEvent>, owners:Query<&Owner>, mut apply_damage_writer:EventWriter<ApplyDamageEvent>) {
    for e in projectile_hit_events.iter() {
        if let Ok(owner) = owners.get_component::<Owner>(e.projectile) {
            if owner.owner != e.target {
                let mut projectile = commands.entity(e.projectile);
                projectile.despawn();
                apply_damage_writer.send(ApplyDamageEvent {
                    amount:100.0,
                    target:e.target
                });
            }
        }
    }
}