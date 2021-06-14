use bevy::{math::{Vec3}, prelude::Entity};
use rand::random;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum BotState {
    Idle,
    RandomRotate,
    Rotate180,
    Exploring
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bot {
    pub next_think:f64,
    pub state:BotState,
    pub mem:[f32;4],
    pub sensors:BotSensors,
    pub attack_timer:f32
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct BotSensors {
    // sensed distance to front obstacle
    pub obstacle_distance_front:f32,
    pub known_enemies:Vec<Enemy>,
    pub visible_enemies:Vec<Enemy>
}

impl BotSensors {
    pub fn get_closest_visible_enemy(&self) -> Option<Enemy> {
        if self.visible_enemies.len() > 0 {
            let mut enemy = self.visible_enemies.first().unwrap();
            for e in &self.visible_enemies {
                if e.distance < enemy.distance {
                    enemy = e;
                }
            }

            return Some(*enemy);
        }

        None
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Enemy {
    pub entity:Entity,
    pub position:Vec3,
    pub distance:f32
}


impl Default for Bot {
    fn default() -> Self {
        Self {
            next_think:random(),
            state:BotState::Idle,
            mem:Default::default(),
            sensors:Default::default(),
            attack_timer:10.0
        }
    }
}