use bevy::prelude::*;

pub struct ApplyDamageEvent {
    pub target:Entity,
    pub amount:f32
}