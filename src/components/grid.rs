use rand::prelude::*;

#[derive(Copy, Clone)]
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
    pub size:usize,
    pub cells:Vec<Cell>,
    pub sheet_width:u32,
    pub sheet_height:u32
}

impl Grid {
    pub fn new(size:usize) -> Grid {
        let mut g = Grid {
            size:size,
            cells:vec![Cell::default(); size * size],
            sheet_width:2,
            sheet_height:2
        };

        return g;
    }

    pub fn randomize(&mut self) {
        let max =self.sheet_height * self.sheet_width;
        let mut rng = rand::thread_rng();
        for cell in &mut self.cells {
            let index:u32 = rng.gen();
            cell.index = index % max;
        }
    }
}