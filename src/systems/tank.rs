use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::Tank;

pub fn tank_system(tank:Query<(&mut Tank, &RigidBodyHandleComponent, Entity)>, mut rigid_body_set:ResMut<RigidBodySet>, time:Res<Time>, mut sprites:Query<&mut TextureAtlasSprite>) {
    tank.for_each_mut(|(mut tank, rigid_body, e)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body.handle()) {
            let dir:Vec2 = [rigid_body.position().rotation.re, rigid_body.position().rotation.im].into();
            let diff = tank.tracks[0] - tank.tracks[1];
            let force = (dir * tank.tracks[0] + dir * tank.tracks[1]) * time.delta_seconds() * 400.0;
            tank.tracks_distance = tank.tracks * time.delta_seconds() + tank.tracks_distance;
            rigid_body.set_angvel(diff, true);
            rigid_body.apply_force([force.x, force.y].into(), true);

            if tank.tracks_distance.length() > 0.2 {
                tank.tracks_distance = [0.0, 0.0].into();
                if let Ok(mut sprite) = sprites.get_component_mut::<TextureAtlasSprite>(e) {
                    sprite.flip_x = !sprite.flip_x;
                }
            }
        }
    });
}