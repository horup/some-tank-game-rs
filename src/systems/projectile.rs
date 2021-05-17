use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{Collider, ColliderHandle, ColliderSet, ContactEvent}};

use crate::{Health, Projectile, ProjectileHitEvent};

pub fn projectile_system(mut commands:Commands, mut projectile_hit_events:EventReader<ProjectileHitEvent>, healths:Query<&Health>) {
    for e in projectile_hit_events.iter() {
        let mut projectile = commands.entity(e.projectile);
        projectile.despawn();
        let mut target = commands.entity(e.target);

        if let Ok(health) = healths.get_component::<Health>(target.id()) {
            target.despawn();
        }
    }
}