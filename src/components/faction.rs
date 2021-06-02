use bevy::prelude::Color;

pub const PLAYER_FACTION:u8 = 0;
pub const ENEMY_FACTION:u8 = 1;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Faction {
    Player,
    EnemyBots
}


impl Faction {
    pub fn primary_color(&self) -> Color {
        match self {
            Faction::Player => return Color::GREEN,
            Faction::EnemyBots => return Color::RED,
        }
    }
}