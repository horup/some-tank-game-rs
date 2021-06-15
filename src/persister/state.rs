use std::collections::HashMap;
use serde::*;
use bevy::prelude::*;
use crate::components::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub bots:HashMap<Entity, Bot>,
}

impl State {
    pub fn _serialize(_world:&mut World) -> State {
        State {
            bots:HashMap::default()
        }
    }

    pub fn _overwrite(_world:&mut World) {
        
    }
}