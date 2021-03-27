use bevy::ecs::{Commands};
use crate::components::State;

pub fn init(commands:&mut Commands) {
    println!("initializing game by spawning a state entity");
    commands.spawn((State {
        in_progress:false,
        timer:0.0
    },));
}