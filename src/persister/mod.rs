use std::collections::VecDeque;
use bevy::{ecs::{component::Component, system::CommandQueue}, prelude::*, utils::HashMap};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::GamePiece;
use crate::components::*;


pub enum PersisterCommand {
    SaveState,
    LoadState
}


#[derive(Default)]
pub struct Persister {
    pub commands:VecDeque<PersisterCommand>
}

impl Persister {
    pub fn push_command(&mut self, command:PersisterCommand) {
        self.commands.push_back(command);
    }
}


fn get_game_pieces(world:&mut World) -> Vec<Entity> {
    let mut game_pieces = world.query::<(Entity, &GamePiece)>();
    let entities:Vec<Entity> = game_pieces.iter(&world).map(|(e, _)| e).collect();
    return entities;
}

fn clear_world(world:&mut World) {
    let entities = get_game_pieces(world);

    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, world);
    for e in entities {
        commands.entity(e).despawn_recursive();
    }

    command_queue.apply(world);
}

fn load_state(world:&mut World) {
    //clear_world(world);
    save_state(world);
}

fn serialize<T:Serialize + Component>(world:&mut World) -> serde_json::Value {

    let mut array:Vec<Value> = Vec::default();
    for (t, _) in world.query::<(&T, &GamePiece)>().iter(world) {
        let value = serde_json::to_value(t).unwrap();
        array.push(value);
    }

    serde_json::Value::Array(array)
}

fn save_state(world:&mut World) {
    serialize::<Tank>(world);
    serialize::<Bot>(world);
}


fn update(world:&mut World) {
    let mut persister = world.get_resource_mut::<Persister>().unwrap();
    if let Some(c) = persister.commands.pop_front() {
        match c {
            PersisterCommand::SaveState => {
                save_state(world);
                return;
            },
            PersisterCommand::LoadState => {
                load_state(world);
                return;
            },
        }
    }
}


pub struct PersisterPlugin;


impl Plugin for PersisterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Persister::default())
        .add_system(update.exclusive_system());
    }
}