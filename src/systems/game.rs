use bevy::prelude::*;
use crate::{NewGameEvent, Thrust, Tile, Tilemap};

pub fn game_system(mut commands: Commands, mut tilemaps:Query<(Entity, &mut Tilemap)>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut meshes: ResMut<Assets<Mesh>>, mut new_game_reader:EventReader<NewGameEvent>) {
    for e in new_game_reader.iter() {
        // desspawn any existing tilemap and children
        for tile_map in tilemaps.iter_mut() {
            commands.entity(tile_map.0).despawn_recursive();
        }
        let tile_map = create_tilemap(e, &mut commands, &asset_server, &mut materials, &mut meshes);
        init_player(e, tile_map, &mut commands, &asset_server, &mut materials, &mut texture_atlases);
    }
}



fn init_player(new_game:&NewGameEvent, tile_map_entity:Entity, mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut materials: &mut ResMut<Assets<StandardMaterial>>, texture_atlases:&mut ResMut<Assets<TextureAtlas>>) {
    let tile_size = Vec2::new(8.0, 8.0);
    let texture_handle = asset_server.load("tanks.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //println!("{:?}", texture_atlases.get_mut(texture_atlas_handle.clone()).unwrap().size);

    let transform = Transform { 
        translation:Vec3::new(0.0, 0.0, 0.0),
        scale:Vec3::splat(1.0 / 8.0),
        ..Default::default()
    };

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas:texture_atlas_handle,
        transform,
        ..Default::default()
    }).insert(Thrust { 
        x:1.0,
        y:0.0
    }).insert(Parent(tile_map_entity));
}


fn create_tilemap(new_game:&NewGameEvent, mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut materials: &mut ResMut<Assets<StandardMaterial>>, mut meshes: &mut ResMut<Assets<Mesh>>) -> Entity
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
