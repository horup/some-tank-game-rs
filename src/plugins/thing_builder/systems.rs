use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use crate::{Drag, Effect, GamePiece, Health, Owner, Projectile, Tank, Turret};

use super::*;

pub fn thing_builder_added_system(mut commands:Commands, query:Query<(Entity, &ThingBuilder), Added<ThingBuilder>>, texture_atlases:Res<TextureAtlases>) {
    query.for_each_mut(|(e, tb)| {
        let mut e = commands.entity(e);
        e.insert(GamePiece::default());
        let mut transform = Transform {
            translation:tb.translation,
            rotation:tb.rotation,
            scale:Vec3::splat(1.0 / 8.0)
        };
        
     

        if let Some(entity) = tb.owner {
            e.insert(Owner::from(entity));
        }

        let x = tb.translation.x;
        let y = tb.translation.y;

        match tb.thing_type {
            ThingType::Unknown => {}
            ThingType::Tank => {
                let rigid_body = RigidBodyBuilder::new_dynamic()
                .translation(x, y)
                .rotation(tb.rotation.angle_between(Quat::default()));
                let collider = ColliderBuilder::cuboid(0.5, 0.5)
                .user_data(e.id().to_bits() as u128);
                e.insert(rigid_body);
                e.insert(collider);
                e.insert(Health::default());
                e.insert(Drag::default());

                let sprite_sheet_bundle = SpriteSheetBundle {
                    texture_atlas:texture_atlases.get_atlas(tb.thing_type),
                    transform,
                    sprite:TextureAtlasSprite {
                        index:texture_atlases.get_index(tb.thing_type),
                        ..Default::default()
                    },
                    ..Default::default()
                };
        
                e.insert_bundle(sprite_sheet_bundle);
                let tank = e.id();

                let turret = commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas:texture_atlases.tanks.clone(),
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
                commands.entity(tank).push_children(&[turret]);
                commands.entity(tank).insert(Tank {
                    turret_entity:turret,
                    tracks:[0.0, 0.0]
                });
            }
            ThingType::Bullet => {
                let speed = 10.0;
                let v = Vec3::new(speed, 0.0, 0.0);
                let v = tb.rotation * v;
                let a = v.angle_between(Vec3::new(1.0, 0.0, 0.0));
                let rigid_body = RigidBodyBuilder::new_dynamic()
                .translation(x, y)
                //.linear_damping(1.5)
                //.angular_damping(1.5)
                .linvel(v.x, v.y)
                .angvel(10.0)
                .rotation(a);

                e.insert(rigid_body);
                
                let collider = ColliderBuilder::cuboid(1.0/8.0, 1.0/8.0)
                .user_data(e.id().to_bits() as u128)
                .modify_solver_contacts(true);
                e.insert(collider);
                e.insert(Projectile::default());

                let sprite_sheet_bundle = SpriteSheetBundle {
                    texture_atlas:texture_atlases.get_atlas(tb.thing_type),
                    transform,
                    sprite:TextureAtlasSprite {
                        index:texture_atlases.get_index(tb.thing_type),
                        ..Default::default()
                    },
                    ..Default::default()
                };
        
                e.insert_bundle(sprite_sheet_bundle);
            }
            ThingType::Effect(effect_type) => {
                match effect_type {
                    EffectType::BulletHit => {
                        transform.scale = transform.scale * 0.25;
                        transform.translation.z = 1.0;
                        e.insert(Effect::new(0.25, 4.0, true).with_start_fade(0.25));

                        let sprite_sheet_bundle = SpriteSheetBundle {
                            texture_atlas:texture_atlases.get_atlas(tb.thing_type),
                            transform,
                            sprite:TextureAtlasSprite {
                                index:texture_atlases.get_index(tb.thing_type),
                                ..Default::default()
                            },
                            ..Default::default()
                        };
                
                        e.insert_bundle(sprite_sheet_bundle);


                    }
                }

                
            }
        }

      
    });
}

pub fn thing_builder_init_system(mut textures:ResMut<TextureAtlases>, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    textures.tanks = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("tanks.png"), Vec2::new(8.0, 8.0), 4, 4));
    textures.white = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("white.png"), Vec2::new(8.0, 8.0), 1, 1));
}