use bevy::prelude::Color;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Faction {
    Greens,
    Reds
}


impl Faction {
    pub fn primary_color(&self) -> Color {
        match self {
            Faction::Greens => return Color::rgb_u8(0, 0xa6, 0x04),
            Faction::Reds => return Color::rgb_u8(0xd7, 0x0, 0x0),
        }
    }
}