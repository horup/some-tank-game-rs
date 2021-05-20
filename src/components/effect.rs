#[derive(Debug, Clone, Copy, Default)]
pub struct Effect {
    pub start:f32,
    pub timer:f32,
    pub grow:f32
}

impl Effect {
    pub fn new(start:f32, grow:f32) -> Effect {
        Effect {
            start,
            timer:start,
            grow
        }
    }
}