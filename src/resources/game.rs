use bevy::core::Time;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum GameState {
    NotSet,
    Loading,
    GetReady,
    Go,
    InProgress,
    Restarting
}

pub struct Game {
    pub state:GameState,
    pub next_state_at:Option<(GameState, f64)>,
}

pub struct GameStateChangeEvent {
    pub from:GameState,
    pub to:GameState
}

impl Game {
    pub fn transition(&mut self, next_state:GameState, in_seconds:f64, time:&Time) {
        self.next_state_at = Some((next_state, time.seconds_since_startup()  + in_seconds));
    }

    pub fn transition_asap(&mut self, next_state:GameState) {
        self.next_state_at = Some((next_state, 0.0));
    }

    pub fn clear_transition(&mut self) {
        self.next_state_at = None;
    }

    pub fn next_state(&self) -> Option<GameState> {
        Some(self.next_state_at?.0)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state:GameState::NotSet,
            next_state_at:Some((GameState::Loading, 0.0))
        }
    }
}