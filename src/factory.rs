use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{Player, Textures, Thrust, Turret};

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
    
        let body = self.commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas:texture_atlas_handle.clone(),
            transform,
            sprite:TextureAtlasSprite {
                index:0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Thrust::default())
        .id();


        let turret = self.commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas:texture_atlas_handle.clone(),
            sprite:TextureAtlasSprite {
                index:1,
                ..Default::default()
            },
            transform:Transform {
                translation:Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Turret::default())
        .id();

        // adding Parent component to the entity above does not work correct due to scale
        // is not properly propagated: https://github.com/bevyengine/bevy/issues/1807
        // can be fixed by doing this instead
        self.commands.entity(body).push_children(&[turret]);
        self.commands.entity(parent).push_children(&[body]);
        body
    }
}