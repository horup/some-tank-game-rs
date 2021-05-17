use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct ProjectileHitEvent {
    pub projectile:Entity,
    pub target:Entity,
    pub location:Vec3
}