use bevy::prelude::Entity;

pub struct Owner {
    pub owner:Entity
}

impl From<Entity> for Owner {
    fn from(e: Entity) -> Self {
        Self { owner: e}
    }
} 