use bevy::{prelude::*, render::camera::{Camera, CameraProjection, OrthographicProjection}};
use crate::components::Tilemap;

pub fn camera_system(mut camera:Query<(&mut OrthographicProjection, &mut Camera, &mut Transform)>, tilemap:Query<&Tilemap>,  windows: ResMut<Windows>) {
    let camera = camera.single_mut();
    let tilemap = tilemap.single();
    let primary = windows.get_primary();
    match (camera, tilemap, primary) {
        (Ok(camera), Ok(tilemap), Some(primary)) => {
            let (mut projection, mut camera, mut transform) = camera;
            //projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;
            projection.scaling_mode = bevy::render::camera::ScalingMode::None;

            let width = primary.width() as usize;
            let height = primary.height() as usize;
            let aspect = width / height;

            let f = 8;
            let s = tilemap.size * f;

            let s = height / s * f;
            let s = s as f32;

            projection.left = -s/2 as f32;
            projection.right = s/2 as f32;
            projection.top = s/2 as f32;
            projection.bottom = -s/2 as f32;

           
            //projection.scale = tilemap.size as f32;
         
            println!("{}", primary.height() );
            transform.translation.x = tilemap.size as f32 / 2.0;
            transform.translation.y = tilemap.size as f32 / 2.0;

            // force update projection matrix without resize
            projection.update(primary.width(), primary.height());
            camera.projection_matrix = projection.get_projection_matrix();
        },
        _ =>{}
    }
}