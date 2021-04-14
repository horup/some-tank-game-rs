use bevy::{prelude::*, render::camera::{Camera, CameraProjection, OrthographicProjection}};
use crate::components::Tilemap;

pub fn camera_system(mut camera:Query<(&mut OrthographicProjection, &mut Camera, &mut Transform)>, tilemap:Query<&Tilemap>,  windows: ResMut<Windows>) {
    let camera = camera.single_mut();
    let tilemap = tilemap.single();

    match (camera, tilemap) {
        (Ok(camera), Ok(tilemap)) => {
            let (mut projection, mut camera, mut transform) = camera;
            projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;
            projection.scale = tilemap.size as f32;
            transform.translation.x = tilemap.size as f32 / 2.0;
            transform.translation.y = tilemap.size as f32 / 2.0;

            // force update projection matrix without resize
            if let Some(primary) = windows.get_primary() {
                projection.update(primary.width(), primary.height());
                camera.projection_matrix = projection.get_projection_matrix();
            }
        },
        _ =>{}
    }
}