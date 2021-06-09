use bevy::prelude::*;

use crate::{Bot, Faction, Player, ThingBuilder, ThingType};

pub struct Spawn {
    pub x:f32,
    pub y:f32,
    pub rotation:f32,
    pub object_type:String
}

pub fn spawn(commands:&mut Commands, spawn:Spawn) {
    let (x, y) = (spawn.x, spawn.y);
    let rotation = Quat::from_rotation_z(spawn.rotation);
    let translation = Vec3::new(x, y, 0.0);
    match spawn.object_type.to_lowercase().as_str() {
        "player" => {
            commands.spawn().insert(ThingBuilder {
                translation,
                rotation,
                thing_type:ThingType::Tank,
                ..Default::default()
            })
            .insert(Player::default())
            .insert(Faction::Greens);
        }
        "bot" => {
            commands.spawn().insert(ThingBuilder {
                translation,
                rotation,
                thing_type:ThingType::Tank,
                ..Default::default()
            })
            .insert(Bot::default())
            .insert(Faction::Reds);
        }
        _ => {}
    }

}