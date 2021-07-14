use std::collections::{HashSet};
use bevy::{asset::Asset, prelude::*};

#[derive(Default)]
pub struct AssetCache {
    loaded:HashSet<HandleUntyped>,
    pub(self) not_loaded:HashSet<HandleUntyped>,
}

impl AssetCache {
    pub fn clear(&mut self) {
        self.loaded.clear();
        self.not_loaded.clear();
    }

    pub fn contains<T:Asset>(&self, handle:&Handle<T>) -> bool {
        let handle = handle.clone_untyped();
        self.loaded.contains(&handle) || self.not_loaded.contains(&handle)
    }

    pub fn is_loaded<T:Asset>(&self, handle:&Handle<T>) -> bool {
        self.loaded.contains(&handle.clone_untyped())
    }

    pub fn remove<T:Asset>(&mut self, handle:&Handle<T>) {
        self.loaded.remove(&handle.clone_untyped());
    }

    pub fn track<T:Asset>(&mut self, handle:&Handle<T>) {
        self.not_loaded.insert(handle.clone_untyped());
    }

    pub fn track_untyped(&mut self, handle:&HandleUntyped) {
        self.not_loaded.insert(handle.clone());
    }

    pub fn count(&self) -> usize {
        self.not_loaded.len() + self.loaded.len()
    }

    pub fn loaded_count(&self) -> usize {
        self.loaded.len()
    }

    pub fn not_loaded_count(&self) -> usize {
        self.not_loaded.len()
    }

    pub fn all_is_loaded(&self) -> bool {
        self.not_loaded.len() == 0
    }
}

fn asset_loader(mut asset_cache:ResMut<AssetCache>, asset_server:Res<AssetServer>) {
    let mut loaded = Vec::new();
    for handle in asset_cache.not_loaded.iter() {
        match asset_server.get_load_state(handle) {
            bevy::asset::LoadState::NotLoaded => {},
            bevy::asset::LoadState::Loading => {},
            bevy::asset::LoadState::Loaded => {
                loaded.push(handle.clone());
            },
            bevy::asset::LoadState::Failed => {},
        }
    }

    for handle in loaded {
        asset_cache.not_loaded.remove(&handle);
        asset_cache.loaded.insert(handle);
    }
}

pub struct AssetCachePlugin;
impl Plugin for AssetCachePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AssetCache::default());
        app.add_system(asset_loader.system());
    }
}


