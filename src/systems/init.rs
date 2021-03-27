use bevy::prelude::*;
use crate::components::{Player, State, Thing};

pub fn init(commands:&mut Commands, asset_server: Res<AssetServer>,  mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("initializing game by spawning non optional entities");

    let texture_handle:Handle<Texture> = asset_server.load("icon.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn((State { in_progress:false, timer:0.0},));

    commands.spawn(Camera2dBundle::default()).spawn(SpriteBundle { 
        material: materials.add(texture_handle.into()), 
        ..Default::default()
    }).with(Player {is_alive:true});
}