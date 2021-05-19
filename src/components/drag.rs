use bevy::math::Vec3;

pub struct Drag {
    pub front:f32,
    pub side:f32
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            front:10.0,
            side:100.0
        }
    }
}