use bevy::ecs::Query;
use bevy::prelude::*;
use crate::components::{State, Thing};

pub fn game(mut query: Query<&mut State>, time:Res<Time>) {
    let state = &mut query.iter_mut().next().unwrap();
    
    state.timer = state.timer + time.delta_seconds();

    if state.in_progress == false {
        if state.timer > 5.0 {
            println!("starting game!");
            state.timer = 0.0;
        }
    }

}