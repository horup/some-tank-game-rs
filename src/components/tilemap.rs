use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub index:u32
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            index:0
        }
    }
}

pub struct Tilemap {
    pub size:usize,
    pub cells:Vec<Tile>,
    pub sheet_width:u32,
    pub sheet_height:u32
}

impl Tilemap {
    pub fn new(size:usize) -> Tilemap {
        let mut g = Tilemap {
            size:size,
            cells:vec![Tile::default(); size * size],
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