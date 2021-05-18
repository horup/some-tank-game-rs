use bevy::math::Vec3;

pub struct Drag {
    pub drag_front:Vec3,
    pub drag_side:Vec3
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            drag_front:Vec3::new(10.0, 0.0, 0.0),
            drag_side:Vec3::new(0.0, 100.0, 0.0)
        }
    }
}