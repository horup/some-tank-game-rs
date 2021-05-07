#[derive(Copy, Clone)]
pub struct Tile {
    pub index:u32,
    pub solid:bool
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            index:0,
            solid:false
        }
    }
}

pub struct Tilemap {
    pub size:usize,
    pub tiles:Vec<Tile>,
    pub sheet_width:u32,
    pub sheet_height:u32,
    pub invalidate:bool
}


impl Tilemap {
    pub fn new(size:usize, sheet_width:u32, sheet_height:u32) -> Tilemap {
        let g = Tilemap {
            size:size,
            tiles:vec![Tile::default(); size * size],
            sheet_width,
            sheet_height,
            invalidate:true
        };

        return g;
    }

    pub fn invalidate(&mut self) {
        self.invalidate = true;
    }

    pub fn set_tile(&mut self, tile:Tile, x:usize, y:usize) {
        self.tiles[y * self.size + x] = tile;
    }

    pub fn get_tile(&mut self, x:usize, y:usize) -> Option<&Tile> {
        let c = self.tiles.get(y * self.size + x);
        return c;
    }
}


pub struct TilemapBuilder {
    
}