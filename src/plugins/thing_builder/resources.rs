use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::ThingType;

#[derive(Default)]
pub struct TextureAtlases {
    pub tanks:Handle<TextureAtlas>
}

impl TextureAtlases {
    pub fn get_atlas(&self, thing_type:ThingType) -> Handle<TextureAtlas> {
        match thing_type {
            ThingType::Unknown => self.tanks.clone(),
            ThingType::Tank => self.tanks.clone(),
            ThingType::Bullet => self.tanks.clone(),
        }
    }

    pub fn get_index(&self, thing_type:ThingType) -> u32 {
        match thing_type {
            ThingType::Unknown => {0}
            ThingType::Tank => {0}
            ThingType::Bullet => {2}
        }
    }
}