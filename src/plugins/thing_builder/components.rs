use bevy::{math::{Quat, Vec3}, prelude::Entity};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThingType {
    Unknown,
    Tank,
    Bullet
}

impl Default for ThingType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Default)]
pub struct ThingBuilder {
    pub translation:Vec3,
    pub rotation:Quat,
    pub thing_type:ThingType,
    pub owner:Option<Entity>
}