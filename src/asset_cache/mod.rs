use std::collections::{HashSet};
use bevy::{asset::Asset, prelude::*};

#[derive(Default)]
pub struct AssetCache {
    assets:HashSet<HandleUntyped>
}

impl AssetCache {
    pub fn cache<T:Asset>(&mut self, handle:&Handle<T>) {
        self.assets.insert(handle.clone_untyped());
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }

    pub fn contains<T:Asset>(&mut self, handle:&Handle<T>) -> bool {
        self.assets.contains(&handle.clone_untyped())
    }

    pub fn remove<T:Asset>(&mut self, handle:&Handle<T>) {
        self.assets.remove(&handle.clone_untyped());
    }
}

pub struct AssetCachePlugin;
impl Plugin for AssetCachePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AssetCache::default());
    }
}


