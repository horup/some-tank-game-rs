use bevy::prelude::*;

#[derive(Default)]
pub struct Textures {
    pub tiles:Handle<Texture>,
    pub tank_atlas:Handle<TextureAtlas>
}

pub fn load_textures_system(mut textures:ResMut<Textures>, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    let tile_size = Vec2::new(8.0, 8.0);

    textures.tiles = asset_server.load("spritesheet3.png");

    textures.tank_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("tanks.png"), tile_size, 4, 4));

    //let tex:Handle<Texture> = Default::default();
}
