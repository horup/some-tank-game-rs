pub struct Health {
    pub amount:i32
}

impl Default for Health {
    fn default() -> Self {
        Health { 
            amount:100
        }
    }
}
