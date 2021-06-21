use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Levels {
    pub maps:Vec<String>
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            maps:vec!["1".into()]
        }
    }

}

impl Levels {
    pub fn count(&self) -> u32 {
        self.maps.len() as u32
    }

    pub fn get_map(&self, level:u32) -> String {
        if let Some(map) = self.maps.get((level -1) as usize) {
            return map.clone();
        }

        return Default::default();
    } 
}