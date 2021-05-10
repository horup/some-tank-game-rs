mod components;
pub use components::*;

mod systems;
pub use systems::*;

use bevy::prelude::*;

use bevy::prelude::CoreStage;



#[derive(Default)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, tilemap_added_system.system());
    }
}