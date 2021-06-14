use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Health {
    pub amount:f32
}

impl Default for Health {
    fn default() -> Self {
        Health { 
            amount:100.0
        }
    }
}
