use bevy::prelude::*;
use crate::{Factory, NewGameEvent, Tile, Tilemap, resources::Textures};

pub fn game_system(mut entities:Query<Entity>, mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut new_game_reader:EventReader<NewGameEvent>, textures:Res<Textures>) {
    for e in new_game_reader.iter() {
     
        // cleanup existing entities
        entities.for_each_mut(|e| {
            let mut e = commands.entity(e);
            e.despawn_recursive();
        });

        // create camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        // create tilemap
        let size = e.map_size;
        let mut tilemap = Tilemap::new(size, 4, 4, "tiles.png");
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
                if y %3 == 0 {
                    if x % (1 + y) == 0 {
                        tilemap.set_tile(Tile {
                            index:1,
                            solid:true,
                            ..Default::default()
                        }, x, y);
                    }
                }
            }
        }


        //let tiles:Vec<Tile> = vec![e.map_size * e.map_size];
       /* commands.spawn().insert(TilemapBuilder {
            sheet_height:
        })*/
        //let tile_map = create_tilemap(e, &mut commands, &asset_server, &mut materials, &mut meshes);


        let mut factory = Factory::new(&mut commands, &textures);

        // spawn a player
        factory.spawn_player(2.5, 2.5);


        // spawn some bots
        for y in 0..10 {
            factory.spawn_bot(10.5, y as f32 + 2.5);
        }

      //  factory.spawn_tank(5.5, 3.5, tile_map);
        //let f = Factory::new(&mut commands, &asset_server);
    }
}
 


fn create_tilemap(new_game:&NewGameEvent, commands: &mut Commands, asset_server: &Res<AssetServer>, mut materials: &mut ResMut<Assets<StandardMaterial>>, mut meshes: &mut ResMut<Assets<Mesh>>) -> Entity
{
   /* let size = new_game.map_size;
    let mut tilemap = Tilemap::new(size, 4, 4);
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
            if y %3 == 0 {
                if x % (1 + y) == 0 {
                    tilemap.set_tile(Tile {
                        index:1,
                        solid:true,
                        ..Default::default()
                    }, x, y);
                }
            }
        }
    }
*/
    Entity::new(0)
    //Tilemap::insert_entity(tilemap, "tiles.png", commands, &asset_server, &mut materials, &mut meshes)
}
