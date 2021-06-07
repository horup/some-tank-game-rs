use std::usize;

pub use bevy::prelude::*;
use crate::{Bot, Faction, GamePiece, Player, ThingBuilder, ThingType, Tile, Tilemap, tiled::TiledMap};
use tiled::Map;

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

fn map_loader(mut map_loader:ResMut<MapLoader>, maps:Res<Assets<TiledMap>>, mut game_pieces:Query<(Entity, &GamePiece)>, mut commands:Commands) {
    if let Some(next_map) = map_loader.next_map.clone() {
        let map = maps.get(next_map.clone());
        if let Some(map) = map {
            
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
                                let mut index = 0;
                                let mut solid = false;

                                let tileset = map.get_tileset_by_gid(gid).expect("tileset was not found");
                                index = gid - tileset.first_gid;
                                if let Some(tile) = tileset.tiles.get(index as usize) {
                                    if let Some(tiled::PropertyValue::BoolValue(property)) = tile.properties.get("solid") {
                                        solid = *property;
                                    }
                                }

                                tilemap.set_tile(Tile {
                                    index,
                                    solid,
                                    ..Default::default()
                                }, col, row);
                            }
                        }
                    },
                    tiled::LayerData::Infinite(_) => panic!("infinite layers not supported"),
                }
            }

            /*
            for y in 0..size {
                tilemap.set_tile(Tile {
                    index:1,
                    solid:true,
                    ..Default::default()
                }, 0, y);
                tilemap.set_tile(Tile {
                    index:1,
                    solid:true,
                    ..Default::default()
                }, size-1, y);
            }

            for x in 0..size {
                tilemap.set_tile(Tile {
                    index:1,
                    solid:true,
                    ..Default::default()
                }, x, 0);
                tilemap.set_tile(Tile {
                    index:1,
                    solid:true,
                    ..Default::default()
                }, x, size - 1);
            }

            for y in 0..size {
                for x in 0..size {
                    if x % 5 == 0 {
                        if y % 5 == 0 {
                            tilemap.set_tile(Tile {
                                index:1,
                                solid:true,
                                ..Default::default()
                            }, x, y);
                        }
                    }
                }
            }*/

            commands.spawn().insert(tilemap).insert(GamePiece::default());

            // spawn player
            commands.spawn().insert(ThingBuilder {
                translation:Vec3::new(2.5, 2.5, 0.0),
                rotation:Quat::default(),
                thing_type:ThingType::Tank,
                ..Default::default()
            })
            .insert(Player::default())
            .insert(Faction::Greens);

            let mut spawn_bot = |x, y| {
                commands.spawn().insert(ThingBuilder {
                    translation:Vec3::new(x, y, 0.0),
                    rotation:Quat::default(),
                    thing_type:ThingType::Tank,
                    ..Default::default()
                })
                .insert(Bot::default())
                .insert(Faction::Reds);
            };

            // spawn bot
            spawn_bot(size as f32 - 2.5, size as f32 - 2.5);
            spawn_bot(2.5, size as f32 - 2.5);
            spawn_bot(size as f32 - 2.5, 2.5);
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