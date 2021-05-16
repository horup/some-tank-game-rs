use bevy::prelude::*;
use bevy_rapier2d::{physics::EventQueue, rapier::geometry::{ColliderHandle, ColliderSet, ContactEvent}};

use crate::Projectile;

pub fn projectile_system(mut events:Res<EventQueue>, collider_set:Res<ColliderSet>, mut commands:Commands, projectiles:Query<(&Projectile,)>) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let colliders = [collider_set.get(h1).unwrap(), collider_set.get(h2).unwrap()];
                for collider in colliders {
                    let e:Entity = Entity::from_bits(collider.user_data as u64);
                    let mut e = commands.entity(e);
                    if let Ok(component) = projectiles.get_component::<Projectile>(e.id()) {
                        //println!("has projectile");
                        e.remove::<Collider>()
                        //e.despawn_recursive();
                    }
                }
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
   /* while let Ok(contact_event) = events.contact_events.pop() {
        if let ContactEvent::Started(handle1, handle2) = contact_event {
            let colliders = [collider_set.get(handle1).unwrap(), collider_set.get(handle2).unwrap()];
            for collider in colliders {
                let e:Entity = Entity::from_bits(collider.user_data as u64);
                let mut e = commands.entity(e);
                if let Ok(component) = projectiles.get_component::<Projectile>(e.id()) {
                    //println!("has projectile");
                    e.despawn_recursive();
                }
            }
        }
    }*/
}