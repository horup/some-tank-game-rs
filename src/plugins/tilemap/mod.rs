mod components;
pub use components::*;

mod systems;
pub use systems::*;

use bevy::prelude::*;



#[derive(Default)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(bevy::prelude::CoreStage::PreUpdate, tilemap_render_system.system());
    }
}