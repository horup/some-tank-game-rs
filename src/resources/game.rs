#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Loading,
    WarmingUp,
    InProgress,
    Restarting
}

pub struct Game {
    pub state:GameState,
    pub next_state:Option<(GameState, f32)>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state:GameState::Loading,
            next_state:None
        }
    }
}