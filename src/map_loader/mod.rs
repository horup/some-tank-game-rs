pub use bevy::prelude::*;
use crate::{GamePiece, Tile, Tilemap, tiled::TiledMap};

mod spawner;
use spawner::*;

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

fn map_loader(mut map_loader:ResMut<MapLoader>, maps:Res<Assets<TiledMap>>, game_pieces:Query<(Entity, &GamePiece)>, mut commands:Commands) {
    if let Some(next_map) = map_loader.next_map.clone() {
        let map = maps.get(next_map.clone());
        if let Some(map) = map {
            map_loader.current_map = Some(next_map.clone());
            map_loader.next_map = None;

            // cleanup existing game pieces
            game_pieces.for_each_mut(|e| {
                let mut e = commands.entity(e.0);
                e.despawn_recursive();
            });

            assert_eq!(map.width, map.height);

            // create tilemap
            let size = map.width as usize;
            let mut tilemap = Tilemap::new(size, 4, "imgs/tiles.png");

            for layer in map.layers.iter() {
                match &layer.tiles {
                    tiled::LayerData::Finite(row) => {
                        for (row, col) in row.iter().enumerate() {
                            for (col, tile) in col.iter().enumerate() {
                                let gid = tile.gid;
                                let mut solid = false;
                                let tileset = map.get_tileset_by_gid(gid).expect("tileset was not found");
                                let id = gid - tileset.first_gid;
                                let tile = tileset.tiles.iter().find(|tile| {
                                    tile.id == id
                                });
                                if let Some(tile) = tile {
                                    if let Some(tiled::PropertyValue::BoolValue(property)) = tile.properties.get("solid") {
                                        solid = *property;
                                    }
                                }

                                // flip row
                                let flipped_row = map.height as usize - row - 1;
                                tilemap.set_tile(Tile {
                                    index: id,
                                    solid,
                                    ..Default::default()
                                }, col, flipped_row);
                            }
                        }
                    },
                    tiled::LayerData::Infinite(_) => panic!("infinite layers not supported"),
                }
            }

            commands.spawn().insert(tilemap).insert(GamePiece::default());

            map.object_groups.iter().for_each(|grp| {
                grp.objects.iter().for_each(|obj| {
                    let x = obj.x + obj.width / 2.0;
                    let y = obj.y - obj.height / 2.0;
                    let x = x / map.tile_width as f32;
                    let y = (map.height as f32 * map.tile_height as f32 - y) / map.tile_height as f32;
                    let gid = obj.gid;
                    let tileset = map.get_tileset_by_gid(gid).expect("tileset was not found");
                    let id = gid - tileset.first_gid;
                    let mut object_type_type = String::default();
                    tileset.tiles.iter().for_each(|tile| {
                        if tile.id == id {
                            object_type_type = tile.tile_type.clone().unwrap_or_default();
                        }
                    });
                    let object_type = if obj.obj_type.len() == 0 {object_type_type} else {obj.obj_type.clone()};
                    let rotation = obj.rotation;
                    spawn(&mut commands, Spawn {
                        x,
                        y,
                        object_type,
                        rotation
                    });
                });
            })
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