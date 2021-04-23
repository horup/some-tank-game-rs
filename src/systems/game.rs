use bevy::prelude::*;
use crate::{Factory, NewGameEvent, Player, Thrust, Tile, Tilemap, resources::Textures};

pub fn game_system(mut commands: Commands, mut tilemaps:Query<(Entity, &mut Tilemap)>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut meshes: ResMut<Assets<Mesh>>, mut new_game_reader:EventReader<NewGameEvent>, textures:Res<Textures>) {
    for e in new_game_reader.iter() {
        // desspawn any existing tilemap and children
        for tile_map in tilemaps.iter_mut() {
            //commands.entity(tile_map.0).despawn_recursive();
        }
        let tile_map = create_tilemap(e, &mut commands, &asset_server, &mut materials, &mut meshes);


        let mut factory = Factory::new(&mut commands, &textures);

        factory.spawn_player(2.5, 2.5, tile_map);



      //  factory.spawn_tank(5.5, 3.5, tile_map);
        //let f = Factory::new(&mut commands, &asset_server);
    }
}
 


fn create_tilemap(new_game:&NewGameEvent, commands: &mut Commands, asset_server: &Res<AssetServer>, mut materials: &mut ResMut<Assets<StandardMaterial>>, mut meshes: &mut ResMut<Assets<Mesh>>) -> Entity
{
    let size = new_game.map_size;
    let mut tilemap = Tilemap::new(size, 4, 4);
    for y in 0..size {
        tilemap.set_tile(Tile {
            index:1
        }, 0, y);
        tilemap.set_tile(Tile {
            index:1
        }, size-1, y);
    }

    for x in 0..size {
        tilemap.set_tile(Tile {
            index:1
        }, x, 0);
        tilemap.set_tile(Tile {
            index:1
        }, x, size - 1);
    }

    Tilemap::insert_entity(tilemap, "tiles.png", commands, &asset_server, &mut materials, &mut meshes)
}
