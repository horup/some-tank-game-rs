use bevy::{math::vec2, prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use crate::components::{Player, State, Thing, Grid};

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut meshes: ResMut<Assets<Mesh>>) {
    println!("initializing game by spawning non optional entities");
    init_camera(&mut commands);
    init_grid(&mut commands, asset_server, materials, texture_atlases, meshes);
}



fn init_grid(mut commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut meshes: ResMut<Assets<Mesh>>)
{
    let size = 16;
    let scale = 32.0;

    let texture_handle:Handle<Texture> = asset_server.load("spritesheet2.png");
    let spritesheet = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16.0, 16.0), 16, 16);
    let spritesheet_handle = texture_atlases.add(spritesheet);
    let mut m:Mesh = Mesh::new(PrimitiveTopology::TriangleList);//shape::Quad::new(quad_width);
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    let mut indicies:Vec<u32> = Vec::new();

    m.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    m.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    m.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    m.set_indices(Some(Indices::U32(indicies)));

    let m = meshes.add(m);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit:true,
        ..Default::default()
    });

    let mut e = commands.spawn();
    let mut g = Grid::new(size);
    let mut test = 0;
    for i in &mut g.cells {
        i.index = test % 4;
        test = test + 1;
    }

    e.insert(g);
    e.insert_bundle(PbrBundle {
        mesh:m.clone(),
        material:material_handle,
        ..Default::default()
    });
    
}

fn init_camera(mut commands: &mut Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
