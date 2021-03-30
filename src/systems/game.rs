use bevy::ecs::Query;
use bevy::prelude::*;
use crate::components::{Player, State, Thing};

pub fn game(mut state_query: Query<&mut State>, time:Res<Time>, mut player_query:Query<&mut Player>) {
   /* let state = &mut state_query.iter_mut().next().unwrap();
    
    state.timer = state.timer + time.delta_seconds();

    if state.in_progress == false {
        if state.timer > 5.0 {
            println!("starting game!");
            state.timer = 0.0;
        }
    }
    else {
        
    }*/

    //println!("{}", time.delta_seconds() * 1000.0);
}