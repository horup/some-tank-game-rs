use std::usize;

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
    tiles:Vec<Tile>,
    sheet_size:u32,
    texture_path:String
}


impl Tilemap {
    pub fn new(size:usize, sheet_size:u32, texture_name:&str) -> Tilemap {
        let g = Tilemap {
            tiles:vec![Tile::default(); size * size],
            sheet_size,
            texture_path:texture_name.into()
        };

        return g;
    }

    pub fn tiles_mut(&mut self) -> &mut[Tile] {
        &mut self.tiles
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    pub fn size(&self) -> usize {
        f32::sqrt(self.tiles.len() as f32) as usize
    }

    pub fn set_tile(&mut self, tile:Tile, x:usize, y:usize) {
        let size = self.size();
        self.tiles[y * size + x] = tile;
    }

    pub fn get_tile(&mut self, x:usize, y:usize) -> Option<&Tile> {
        let c = self.tiles.get(y * self.size() + x);
        return c;
    }

    pub fn texture_path(&self) -> &str {
        &self.texture_path
    }

    pub fn sheet_size(&self) -> u32 {
        self.sheet_size
    }
}