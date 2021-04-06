pub struct Cell {
    pub index:u32
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            index:0
        }
    }
}

pub struct Grid {
    pub size:u32,
    pub cells:Vec<Cell>
}

impl Grid {
    pub fn new(size:u32) -> Grid {
        Grid {
            size:size,
            cells:vec![]
        }
    }
}