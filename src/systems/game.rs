use bevy::prelude::*;
use crate::{GamePiece, NewGameEvent, Player, ThingBuilder, ThingType, Tile, Tilemap, resources::Textures};

pub fn game_system(mut game_pieces:Query<(Entity, &GamePiece)>, mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut new_game_reader:EventReader<NewGameEvent>, textures:Res<Textures>) {
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
            thing_type:ThingType::Tank,
            ..Default::default()
        })
        .insert(Player::default());

        //let mut factory = Factory::new(&mut commands, &textures);

        // spawn a player
        //factory.spawn_player(2.5, 2.5);


        // spawn some bots
       /* for y in 0..100 {
            factory.spawn_bot(10.5, y as f32 + 2.5);
        }*/

      //  factory.spawn_tank(5.5, 3.5, tile_map);
        //let f = Factory::new(&mut commands, &asset_server);
    }
}
 


fn create_tilemap(new_game:&NewGameEvent, commands: &mut Commands, asset_server: &Res<AssetServer>, mut materials: &mut ResMut<Assets<StandardMaterial>>, mut meshes: &mut ResMut<Assets<Mesh>>) -> Entity
{
   /* let size = new_game.map_size;
    let mut tilemap = Tilemap::new(size, 4, 4);
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
            if y %3 == 0 {
                if x % (1 + y) == 0 {
                    tilemap.set_tile(Tile {
                        index:1,
                        solid:true,
                        ..Default::default()
                    }, x, y);
                }
            }
        }
    }
*/
    Entity::new(0)
    //Tilemap::insert_entity(tilemap, "tiles.png", commands, &asset_server, &mut materials, &mut meshes)
}
