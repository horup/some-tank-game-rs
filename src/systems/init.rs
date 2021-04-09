use bevy::{math::vec2, prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use crate::components::{Player, State, Thing, Tilemap};

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut meshes: ResMut<Assets<Mesh>>) {
    println!("initializing game by spawning non optional entities");
    init_camera(&mut commands);
    init_grid(&mut commands, asset_server, materials, meshes);
}


fn init_grid(mut commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>)
{
    let mut g = Tilemap::new(8, 4, 4);
    g.randomize();
    Tilemap::insert_entity(g, "spritesheet3.png", commands, asset_server, materials, meshes);
}

fn init_camera(mut commands: &mut Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
