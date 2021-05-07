use bevy::{math::vec2, prelude::*, render::mesh::Indices};
use bevy_rapier2d::{na::Isometry2, rapier::{dynamics::RigidBodyBuilder, geometry::{ColliderBuilder, SharedShape}}};

use super::Tilemap;


pub fn tilemap_render_system(mut meshes: ResMut<Assets<Mesh>>, query: Query<(&mut Tilemap, &mut Handle<Mesh>, Entity)>, mut commands:Commands) {
    query.for_each_mut(|(mut tilemap, mesh, entity)| {
        let m = meshes.get_mut(mesh.id).expect("mesh was not found for grid");
        let scale = 1.0;
        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        let mut indicies:Vec<u32> = Vec::new();
        let size = tilemap.size;
        let mut i = 0;
        for y in 0..size {
            for x in 0..size {
                let index = y * size + x;
                let cell = tilemap.tiles.get(index).expect("grid was out of bounds");
                let north_west = vec2(x as f32 * scale, y as f32 * scale + scale);
                let north_east = vec2(x as f32 * scale + scale, y as f32 * scale + scale);
                let south_west = vec2(x as f32 * scale,  y as f32 * scale);
                let south_east = vec2(x as f32 * scale + scale, y as f32 * scale);

                let tex_w = 1.0 / tilemap.sheet_width as f32;
                let tex_h = 1.0 / tilemap.sheet_height as f32;
                let x = (cell.index % tilemap.sheet_width) as f32; 
                let y = (cell.index / tilemap.sheet_height) as f32;

                // alpha is used to shift the texture samples 'a bit inwards' to avoid artifacts when rendering
                // different resolutions now power of 2
                let alpha = 0.001;
                let u = x * tex_w;
                let v = y * tex_h;

                let vertices = [
                    (
                        [south_west.x, south_west.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + alpha, v+tex_h - alpha],
                    ),
                    (
                        [north_west.x, north_west.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + alpha, v + alpha],
                    ),
                    (
                        [north_east.x, north_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u+tex_w - alpha, v + alpha],
                    ),
                    (
                        [south_east.x, south_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u+tex_w - alpha, v+tex_h - alpha],
                    ),
                ];
            
                for (position, normal, uv) in vertices.iter() {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                indicies.push(i + 0);
                indicies.push(i + 2);
                indicies.push(i + 1);
                indicies.push(i + 0);
                indicies.push(i + 3);
                indicies.push(i + 2);

                i += 4;
            }
        }

        m.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        m.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        m.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        m.set_indices(Some(Indices::U32(indicies)));

        if tilemap.invalidate {
            tilemap.invalidate = false;

            let mut e = commands.entity(entity);
            e.remove::<i32>();
            let mut shapes = Vec::new();
            for y in 0..tilemap.size {
                for x in 0..tilemap.size {
                    if tilemap.get_tile(x, y).unwrap().solid {
                        shapes.push((Isometry2::new([x as f32 + 0.5, y as f32 + 0.5].into(), 0.0), SharedShape::cuboid(0.5, 0.5)));
                    }
                }
            }

            if shapes.len() > 0 {
                let compound = SharedShape::compound(shapes);
                let collider = ColliderBuilder::new(compound);
                let rigid_body = RigidBodyBuilder::new_static();
                e.insert(rigid_body);
                e.insert(collider);
            }
        }
    });
}