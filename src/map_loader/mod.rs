pub use bevy::prelude::*;
use crate::tiled::TiledMap;

#[derive(Default)]
pub struct MapLoader {
    pub(in self) next_map:Option<Handle<TiledMap>>,
    pub(in self) current_map:Option<Handle<TiledMap>>
}

impl MapLoader {
    pub fn load_map(&mut self, map_path:&str, asset_server:Res<AssetServer>) {
        self.next_map = Some(asset_server.load(map_path));
    }
}

fn map_loader(mut map_loader:ResMut<MapLoader>, maps:Res<Assets<TiledMap>>) {
    if let Some(next_map) = map_loader.next_map.clone() {
        let map = maps.get(next_map.clone());
        if let Some(map) = map {
            map_loader.current_map = Some(next_map);
            map_loader.next_map = None;
        }
    }
}
pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MapLoader::default());
        app.add_system(map_loader.system());
    }
}