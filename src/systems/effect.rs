use bevy::prelude::*;
use crate::Effect;


pub fn effect_system(mut commands:Commands, query:Query<(Entity, &mut Effect, &mut Transform)>, mut sprite:Query<&mut TextureAtlasSprite>, time:Res<Time>) {
    query.for_each_mut(|(e, mut effect, mut transform)| {
        if effect.timer == effect.start {
            effect.start_scale = transform.scale;
        }
        let change = time.delta_seconds();
        effect.timer -= change;
        if effect.timer <= 0.0 {
            effect.timer = 0.0;
        }
        let elapsed = 1.0 - effect.timer / effect.start;
        if effect.scale_factor != 0.0 {
            transform.scale = effect.start_scale + effect.start_scale * elapsed * effect.scale_factor;
        }
        if effect.fade {
            if let Ok(mut sprite) = sprite.get_component_mut::<TextureAtlasSprite>(e) {
                if elapsed > effect.start_fade && effect.start_fade < 1.0 {
                    let a = 1.0 - (elapsed - effect.start_fade) / (1.0 - effect.start_fade);
                    sprite.color.set_a(a);
                } else {
                    
                }


            }
        }
        if effect.timer <= 0.0 {
            commands.entity(e).despawn_recursive();
        }
    });
}