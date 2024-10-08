use crate::modules::tile;
use crate::modules::map_attrs;
use rand::Rng;

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Vec<tile::Tile>>
}

impl Default for Map {
    fn default() -> Self {
        Map::new(&map_attrs::MapAttrs {
            ..Default::default()
        })
    }
}

impl Map {
    pub fn new(map_attributes: &map_attrs::MapAttrs) -> Map {
        let map_tiles = vec![vec![tile::Tile::new(&map_attributes.base_tile_height); map_attributes.width]; map_attributes.length];
        Map {
            tiles: map_tiles
        }
    }

    pub fn generate_map(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                //temporary, just to create something with the map
                let a_height: i32 = rand::thread_rng().gen_range(1..256);
                tile.set_height(a_height);
            }
        }
    }

    pub fn get_tiles(&self) -> &Vec<Vec<tile::Tile>> {
        &self.tiles
    }

    pub fn set_tiles(&mut self, map_tiles: Vec<Vec<tile::Tile>>) {
        self.tiles = map_tiles;
    }

    pub fn get_tile(&self, row: usize, col: usize) -> &tile::Tile {
        &self.tiles[row][col]
    }

    //this will be used when needing to completely change a tile, rather than any time any
    //of one's data changes
    pub fn set_tile(&mut self, a_tile: tile::Tile, row: usize, col: usize) {
        self.tiles[row][col] = a_tile
    }
}