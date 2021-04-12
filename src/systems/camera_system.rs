use bevy::{prelude::{Query, Transform}, render::camera::{Camera, OrthographicProjection}};

use crate::components::Tilemap;

pub fn camera_system(mut camera:Query<(&mut OrthographicProjection, &mut Camera, &mut Transform)>, tilemap:Query<&Tilemap>) {

    let (mut projection, camera, mut transform) = camera.single_mut().unwrap();
    let tilemap = tilemap.single().unwrap();
    projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;
    projection.scale = tilemap.size as f32;

    transform.translation.x = tilemap.size as f32 / 2.0;
    transform.translation.y = tilemap.size as f32 / 2.0;

}