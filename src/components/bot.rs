use bevy::{math::{Vec2, Vec3}, prelude::Entity};
use rand::random;

#[derive(Clone, Copy)]
pub enum BotState {
    Idle,
    RandomRotate,
    Rotate180,
    Exploring
}

pub struct Bot {
    pub next_think:f64,
    pub state:BotState,
    pub mem:[f32;4],
    pub visible_target:Option<Vec2>,
    pub sensors:BotSensors
}

#[derive(Default)]
pub struct BotSensors {
    // sensed distance to front obstacle
    pub obstacle_distance_front:f32,
    pub known_enemies:Vec<Enemy>,
    pub visible_enemies:Vec<Enemy>
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub entity:Entity,
    pub position:Vec3
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