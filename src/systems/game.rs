use std::f32::consts::PI;

use bevy::prelude::*;
use crate::{Bot, GamePiece, NewGameEvent, Player, ThingBuilder, ThingType, Tile, Tilemap};

pub fn game_system(game_pieces:Query<(Entity, &GamePiece)>, mut commands: Commands, asset_server: Res<AssetServer>, mut new_game_reader:EventReader<NewGameEvent>) {
    for e in new_game_reader.iter() {
        // cleanup existing entities
        game_pieces.for_each_mut(|e| {
            let mut e = commands.entity(e.0);
            e.despawn_recursive();
        });
      
        // UI camera
        commands.spawn_bundle(UiCameraBundle::default());

        // create camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GamePiece::default());

        // spawn fps:
        commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Blueprint 3.0",
                TextStyle {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });


        // create tilemap
        let size = e.map_size;
        let mut tilemap = Tilemap::new(size, 4, "tiles.png");
        for y in 0..size {
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, 0, y);
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, size-1, y);
        }

        for x in 0..size {
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, x, 0);
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, x, size - 1);
        }

        for y in 0..size {
            for x in 0..size {
                if x % 5 == 0 {
                    if y % 5 == 0 {
                        tilemap.set_tile(Tile {
                            index:1,
                            solid:true,
                            ..Default::default()
                        }, x, y);
                    }
                }
            }
        }

        commands.spawn().insert(tilemap);

        // spawn player
        commands.spawn().insert(ThingBuilder {
            translation:Vec3::new(2.5, 2.5, 0.0),
            rotation:Quat::default(),
            thing_type:ThingType::Tank,
            ..Default::default()
        })
        .insert(Player::default());

        // spawn bot
        commands.spawn().insert(ThingBuilder {
            translation:Vec3::new(size as f32 - 2.5, size as f32 - 2.5, 0.0),
            thing_type:ThingType::Tank,
            rotation:Quat::from_rotation_z(PI),
            ..Default::default()
        })
        .insert(Bot::default());
    }
}
 
