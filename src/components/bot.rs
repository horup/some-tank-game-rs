use bevy::math::Vec2;
use rand::random;

#[derive(Clone, Copy)]
pub enum BotState {
    Idle,
    RandomRotate,
    Rotate180,
    Exploring
}

#[derive(Clone, Copy)]
pub struct Bot {
    pub next_think:f64,
    pub state:BotState,
    pub mem:[f32;4],
    pub visible_target:Option<Vec2>,
    pub sensors:BotSensors
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BotSensors {
    // sensed distance to front obstacle
    pub obstacle_distance_front:f32
}


impl Default for Bot {
    fn default() -> Self {
        Self {
            next_think:random(),
            state:BotState::Idle,
            mem:Default::default(),
            visible_target:None,
            sensors:Default::default()
        }
    }
}