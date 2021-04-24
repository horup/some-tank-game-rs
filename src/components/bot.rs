use rand::random;


#[derive(Clone, Copy)]
pub struct Bot {
    pub next_think:f64
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            next_think:random()
        }
    }
}