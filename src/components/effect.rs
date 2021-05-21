use bevy::math::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Effect {
    pub start:f32,
    pub timer:f32,
    pub scale_factor:f32,
    pub fade:bool,
    pub start_scale:Vec3,
    pub start_fade:f32
}

impl Effect {
    pub fn new(timer_sec:f32, scale_factor:f32, fade:bool) -> Effect {
        Effect {
            start: timer_sec,
            timer:timer_sec,
            scale_factor,
            fade,
            start_scale:Vec3::new(1.0, 1.0, 1.0),
            start_fade:0.0
        }
    }

    pub fn with_start_fade(mut self, start_fade:f32) -> Self {
        self.start_fade = start_fade;
        self 
    }
}