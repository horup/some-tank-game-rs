use bevy::prelude::*;
use super::*;

pub fn sprite_builder_added_system(mut commands:Commands, mut query:Query<(Entity, &ThingBuilder), Added<ThingBuilder>>, texture_atlases:Res<TextureAtlases>) {
    query.for_each_mut(|(e, sb)| {
        let mut e = commands.entity(e);
        let transform = Transform {
            translation:sb.translation,
            rotation:sb.rotation,
            scale:Vec3::splat(1.0 / 8.0)
        };
        
        let sprite_sheet_bundle = SpriteSheetBundle {
            texture_atlas:texture_atlases.get_atlas(sb.thing_type),
            transform,
            sprite:TextureAtlasSprite {
                index:texture_atlases.get_index(sb.thing_type),
                ..Default::default()
            },
            ..Default::default()
        };

        e.insert_bundle(sprite_sheet_bundle);
    });
}

pub fn thing_builder_init_system(mut textures:ResMut<TextureAtlases>, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    textures.tanks = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("tanks.png"), Vec2::new(8.0, 8.0), 4, 4));
}