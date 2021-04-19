use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{Player, Textures, Thrust};

pub struct Factory<'a, 'b: 'a, 'c : 'a> {
    pub commands:&'a mut Commands<'b>,
    pub textures:&'c Textures
}

impl<'a, 'b, 'c, 'd> Factory<'a, 'b, 'c> {
    pub fn new(commands:&'a mut Commands<'b>, textures:&'c Textures) -> Self {
        Self {
            commands,
            textures
        }
    }


    pub fn spawn_player(&mut self, x:f32, y:f32, parent:Entity) -> Entity {
        let tank = self.spawn_tank(x, y, parent);
        self.commands.entity(tank)
        .insert(Player { 

        })
        .id()
    }

    pub fn spawn_tank(&mut self, x:f32, y:f32, parent:Entity) -> Entity {
        let texture_atlas_handle = self.textures.tank_atlas.clone();

        let transform = Transform { 
            translation:Vec3::new(x, y, 0.0),
            scale:Vec3::splat(1.0 / 8.0),
            ..Default::default()
        };
    
        self.commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas:texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Thrust::default())
        .insert(Parent(parent))
        .id()
    }
}