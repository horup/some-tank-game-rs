use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{ColliderSet, ContactEvent}};

use crate::{Projectile, ProjectileHitEvent};

pub fn physics_system(mut physics_events:Res<EventQueue>, collider_set:Res<ColliderSet>, mut commands:Commands, projectiles:Query<&Projectile>, mut projectile_hit_events:EventWriter<ProjectileHitEvent>) {
    while let Ok(contact_event) = physics_events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                if let (Some(h1), Some(h2)) = (collider_set.get(h1), collider_set.get(h2)) {
                    let colliders = [(h1, h2), (h2, h1)];
                    for (col1, col2) in &colliders {
                        let col1:Entity = Entity::from_bits(col1.user_data as u64);
                        let col2:Entity = Entity::from_bits(col2.user_data as u64);
                      
                        let mut e = commands.entity(col1);
                        
                        if let Ok(component) = projectiles.get_component::<Projectile>(e.id()) {
                            projectile_hit_events.send(ProjectileHitEvent {
                                projectile:e.id(),
                                target:col2,
                                location:Vec3::default()
                            })
                        }
                    }
                }
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
}