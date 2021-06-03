use bevy::{math::Vec2, prelude::Entity};


#[derive(Copy, Clone)]
pub struct Tank {
    pub tracks:Vec2,
    pub turret_entity:Entity,
    pub tracks_distance:Vec2
}

impl Tank {
    pub fn new(turret_entity:Entity) -> Self {
        Self {
            tracks:Vec2::default(),
            turret_entity,
            tracks_distance:Vec2::default()
        }
    }
}