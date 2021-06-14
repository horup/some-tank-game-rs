use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Drag {
    pub front:f32,
    pub side:f32
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            front:5.0,
            side:10.0
        }
    }
}