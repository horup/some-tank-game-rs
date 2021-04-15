pub struct NewGameEvent {
    pub map_size:usize
} 

impl Default for NewGameEvent {
    fn default() -> Self {
        NewGameEvent {
            map_size:16
        }
    }
}