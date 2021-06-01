use bevy::prelude::*;

mod new_game;
pub use new_game::*;

mod projectile_hit;
pub use projectile_hit::*;

mod apply_damage;
pub use apply_damage::*;

use crate::resources::GameStateChangeEvent;

#[derive(Default)]
pub struct EventsPlugin {
}

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_event::<NewGameEvent>()
        .add_event::<ProjectileHitEvent>()
        .add_event::<ApplyDamageEvent>()
        .add_event::<GameStateChangeEvent>();
    }
}