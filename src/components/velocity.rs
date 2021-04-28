use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct Velocity {
    pub velocity:Vec3
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            velocity:Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
