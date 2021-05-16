use bevy::prelude::*;

mod components;
pub use components::*;

mod systems;
pub use systems::*;

mod resources;
pub use resources::*;

#[derive(Default)]
pub struct SpriteBuilderPlugin {

}

impl Plugin for SpriteBuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(TextureAtlases::default())
        .add_system_to_stage(CoreStage::PostUpdate, thing_builder_added_system.system())
        .add_startup_system(thing_builder_init_system.system());
    }
}