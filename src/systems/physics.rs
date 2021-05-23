use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{ColliderSet, ContactEvent, NarrowPhase}};

use crate::{Projectile, ProjectileHitEvent};

pub fn physics_system(mut physics_events:Res<EventQueue>, collider_set:Res<ColliderSet>, mut commands:Commands, projectiles:Query<&Projectile>, mut projectile_hit_events:EventWriter<ProjectileHitEvent>, narrow_set:Res<NarrowPhase>) {
    while let Ok(contact_event) = physics_events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                if let (Some(col1), Some(col2)) = (collider_set.get(h1), collider_set.get(h2)) {
                    let colliders = [(col1, col2), (col2, col1)];
                    for (col1, col2) in &colliders {

                        if let Some(contact_pair) = narrow_set.contact_pair(h1, h2) {
                            if contact_pair.has_any_active_contact {
                                let col1_translation = col1.position().translation;
                                let col1:Entity = Entity::from_bits(col1.user_data as u64);
                                let col2:Entity = Entity::from_bits(col2.user_data as u64);
                                if let Some(contact) = contact_pair.find_deepest_contact() {
                                    let mut world_point:Vec3 = Vec3::default();
                                    for p in &contact.0.data.solver_contacts {
                                        world_point = [p.point.x, p.point.y, 0.0].into();
                                    }

                                    let mut e = commands.entity(col1);
                                    if let Ok(component) = projectiles.get_component::<Projectile>(e.id()) {

                                        projectile_hit_events.send(ProjectileHitEvent {
                                            projectile:e.id(),
                                            target:col2,
                                            location:world_point
                                        })
                                    }
                                }
                            }
                        }

                    }
                }
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
}