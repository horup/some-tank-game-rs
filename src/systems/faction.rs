use bevy::prelude::*;

use crate::Faction;

pub fn faction_system(mut query:Query<(&Faction, &mut TextureAtlasSprite)>) {
    query.for_each_mut(|(faction, mut sprite)| {
        if sprite.color != faction.primary_color() {
            sprite.color = faction.primary_color();
        }
    });
}