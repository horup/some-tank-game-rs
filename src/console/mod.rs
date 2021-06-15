use std::collections::VecDeque;

use bevy::prelude::*;

mod console_command;
pub use console_command::*;

use crate::{MapLoader, Persister, PersisterCommand};


#[derive(Default)]
pub struct Console {
    pub(in self) command_queue:VecDeque<ConsoleCommand>
}

impl Console {
    pub (in self) fn pop_command(&mut self) -> Option<ConsoleCommand> {
        self.command_queue.pop_front()
    }

    pub fn push_command(&mut self, cmd:ConsoleCommand) {
        println!("> {:?}", cmd);
        self.command_queue.push_back(cmd);
    }
    pub fn load_map(&mut self, map_name:&str) {
        self.push_command(ConsoleCommand::LoadMap(map_name.into()));
    }
    pub fn load_state(&mut self, index:u8) {
        self.push_command(ConsoleCommand::LoadState(index));
    }
    pub fn save_state(&mut self, index:u8) {
        self.push_command(ConsoleCommand::SaveState(index));
    }
}

pub fn command_interpreter(mut persister:ResMut<Persister>, mut console:ResMut<Console>, asset_server:Res<AssetServer>, mut map_loader:ResMut<MapLoader>) {
    if let Some(command) = console.pop_command() {
        match command {
            ConsoleCommand::LoadMap(map_name) => {
                let path:String = "maps/".to_owned() + &map_name + ".tmx";
                map_loader.load_map(&path, asset_server);
            }
            ConsoleCommand::SaveState(_index) => {
                persister.push_command(PersisterCommand::SaveState);
            },
            ConsoleCommand::LoadState(_index) => {
                persister.push_command(PersisterCommand::LoadState);
            },
        }
    }
}


pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Console::default());
        app.add_system(command_interpreter.system());
    }
}

