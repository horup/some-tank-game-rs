pub const PLAYER_FACTION:u8 = 0;
pub const ENEMY_FACTION:u8 = 1;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]

pub struct Faction(pub u8);