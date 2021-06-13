
#[derive(Debug)]
pub enum ConsoleCommand {
    LoadMap(String),
    SaveState(u8),
    LoadState(u8)
}