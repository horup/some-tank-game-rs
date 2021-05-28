use bevy::prelude::Entity;


#[derive(Copy, Clone)]
pub struct Tank {
    pub tracks:[f32;2],
    pub turret_entity:Entity
}