use bevy::{prelude::*, utils::HashMap};

use crate::{GamePiece, Waypoint, WaypointEvent};

#[derive(Default)]
pub struct WaypointMarkerSprites { 
    pub sprites:HashMap<Vec3, Entity>
}

pub fn waypoints_marker_system(mut commands:Commands, mut waypoint_event_reader:EventReader<WaypointEvent>, asset_server:Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>, query:Query<(Entity, &Waypoint)>) {
    for e in waypoint_event_reader.iter() {
        match e {
            WaypointEvent::Added(w) => {
                let mut l = w.location;
                l.z = 0.0;
                let texture_handle = asset_server.load("imgs/waypoint.png");
                let mut transform = Transform::from_translation(l);
                let s = 1.0/8.0;
                transform.scale = Vec3::new(s, s, s);
                commands.spawn_bundle(SpriteBundle {
                    transform,
                    material: materials.add(texture_handle.into()),
                    ..Default::default()
                })
                .insert(GamePiece)
                .insert(*w);
            },
            WaypointEvent::Removed(w) => {
                for e in query.iter() {
                    if *e.1 == *w {
                        commands.entity(e.0).despawn_recursive();
                    }
                }
            },
            WaypointEvent::Clear => {
                for e in query.iter() {
                        commands.entity(e.0).despawn_recursive();
                }
            },
        }
       
    }
}