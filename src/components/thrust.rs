use bevy::math::Vec3;

#[derive(Clone, Copy)]
pub struct Thrust {
    pub force:Vec3,
    /**True if thrust is constrained to only one axis at a time */
    pub constrained:bool
}

impl Default for Thrust {
    fn default() -> Self {
        Self {
            force:Vec3::new(0.0, 0.0, 0.0),
            constrained:true
        }
    }
}
