use std::collections::VecDeque;

use bevy::prelude::*;


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


fn update(world:&mut World) {
    let mut persister = world.get_resource_mut::<Persister>().unwrap();
    while let Some(c) = persister.commands.pop_front() {
        println!("command!");
    }
}


pub struct PersisterPlugin;


impl Plugin for PersisterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Persister::default())
        .add_system(update.exclusive_system());
    }
}