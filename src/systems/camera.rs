use bevy::{prelude::*, render::camera::{Camera, CameraProjection, OrthographicProjection}};
use crate::components::Tilemap;

pub fn camera_system(mut camera:Query<(&mut OrthographicProjection, &mut Camera, &mut Transform)>, tilemap:Query<&Tilemap>,  windows: ResMut<Windows>) {
    let camera = camera.single_mut();
    let tilemap = tilemap.single();
    let primary = windows.get_primary();
    match (camera, tilemap, primary) {
        (Ok(camera), Ok(tilemap), Some(primary)) => {
            let (mut projection, mut camera, mut transform) = camera;
            
            projection.scaling_mode = bevy::render::camera::ScalingMode::None;


            // calculate pixel perfect integer scaling
            let tile_size = 8;
            let area_width = primary.width() as u32;
            let area_height = primary.height() as u32;

            let fraction_x = (area_width as f32 / 2.0).fract();
            let fraction_y = (area_width as f32 / 2.0).fract() > 0.0;


            let tilemap_width = tilemap.size as u32;
            let tilemap_height = tilemap.size as u32;
            let tilemap_width_px = (tilemap.size * tile_size) as u32;
            let tilemap_height_px = (tilemap.size * tile_size) as u32;

            let tilemap_integer_size = integer_scaling::calculate_size(area_width, area_height, tilemap_width_px, tilemap_height_px);
            projection.right = tilemap_width as f32 * area_width as f32 / tilemap_integer_size.width as f32;
            projection.top = tilemap_height as f32 * area_height as f32 / tilemap_integer_size.height as f32;
            
            // shift optimum projection
            projection.left = -projection.right / 2.0;
            projection.bottom = -projection.top / 2.0;
            projection.right /= 2.0;
            projection.top /= 2.0;

            // move the camera to the center of the tilemap
            transform.translation.x = tilemap.size as f32 / 2.0;
            transform.translation.y = tilemap.size as f32 / 2.0;

            // force update projection matrix without resize
            projection.update(primary.width(), primary.height());
            camera.projection_matrix = projection.get_projection_matrix();
        },
        _ =>{}
    }
}