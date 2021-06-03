use bevy::prelude::*;

use crate::Faction;

pub fn faction_system(query:Query<(Entity, &Faction)>, children:Query<&Children>, mut sprites:Query<&mut TextureAtlasSprite>) {
    query.for_each_mut(|(e, faction)| {
        if let Ok(mut sprite) = sprites.get_component_mut::<TextureAtlasSprite>(e) {
            set_color(faction, &mut sprite);
        }
        if let Ok(children) = children.get_component::<Children>(e) {
            for child in children.iter() {
                if let Ok(mut sprite) = sprites.get_component_mut::<TextureAtlasSprite>(*child) {
                    set_color(faction, &mut sprite);
                }
            }
        }
    });
}

fn set_color(faction:&Faction, sprite:&mut Mut<TextureAtlasSprite>) {
    if sprite.color != faction.primary_color() {
        sprite.color = faction.primary_color();
    }
}