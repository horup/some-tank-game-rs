use rand::random;

#[derive(Clone, Copy)]
pub enum BotState {
    Idle,
    RandomRotate,
    Exploring
}

#[derive(Clone, Copy)]
pub struct Bot {
    pub next_think:f64,
    pub state:BotState
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            next_think:random(),
            state:BotState::Idle
        }
    }
}