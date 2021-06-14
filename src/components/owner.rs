use serde::{Serialize, Deserialize};
use bevy::prelude::Entity;

#[derive(Serialize, Deserialize)]
pub struct Owner {
    pub owner:Entity
}

impl From<Entity> for Owner {
    fn from(e: Entity) -> Self {
        Self { owner: e}
    }
} 