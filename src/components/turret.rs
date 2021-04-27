use bevy::math::Vec3;

#[derive(Copy, Clone)]
pub struct  Turret {
    pub target:Vec3,
    pub cooldown:f32,
    pub trigger:bool
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            target:Vec3::default(),
            cooldown:0.0,
            trigger:false
        }
    }
}