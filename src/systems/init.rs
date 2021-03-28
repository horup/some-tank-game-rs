use bevy::prelude::*;
use crate::components::{Player, State, Thing};

pub fn init(commands:&mut Commands, asset_server: Res<AssetServer>,  mut materials: ResMut<Assets<ColorMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    println!("initializing game by spawning non optional entities");

    
    commands.spawn(Camera2dBundle::default());
    commands.spawn((State { in_progress:false, timer:0.0},));


    let texture_handle:Handle<Texture> = asset_server.load("spritesheet.png");
    let spritesheet = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let spritesheet_handle = texture_atlases.add(spritesheet);
   
    let l = 32;
    for y in 0..l {
        for x in 0..l {
          
            commands.spawn(SpriteSheetBundle {
                texture_atlas:spritesheet_handle.clone(),
                transform:Transform::from_translation(Vec3::new((x as f32) * 16.0, (y as f32) * 16.0, 0.0)),
                ..Default::default()
            }).with(Player {is_alive:true});
        }
    }
   
   
}