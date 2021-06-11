use bevy::prelude::*;
use crate::{AppState, Bot, Console, Hud, Player};

use bevy::core::Time;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]

pub struct GameDirector {
    pub quick:bool,
    pub level:i32,
    pub levels:i32
}

impl Default for GameDirector {
    fn default() -> Self {
        Self {
            quick:false,
            level:1,
            levels:4
        }
    }
}

pub struct GameDirectorPlugin;

impl Plugin for GameDirectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(GameDirector::default());
    }
}