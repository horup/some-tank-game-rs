use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{Collider, ColliderHandle, ColliderSet, ContactEvent}};

use crate::{Health, Owner, Projectile, ProjectileHitEvent};

pub fn projectile_system(mut commands:Commands, mut projectile_hit_events:EventReader<ProjectileHitEvent>, healths:Query<&Health>, owners:Query<&Owner>) {
    for e in projectile_hit_events.iter() {
        if let Ok(owner) = owners.get_component::<Owner>(e.projectile) {
            println!("{:?},{:?}", owner.owner, e.target);
            if owner.owner != e.target {
                let mut projectile = commands.entity(e.projectile);
                projectile.despawn();
                if let Ok(health) = healths.get_component::<Health>(e.target) {
                    let mut target = commands.entity(e.target);
                    target.despawn_recursive();
                }
            }
        }
    }
}