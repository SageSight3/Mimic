use crate::modules::tile::Tile;
use crate::modules::map_attrs::MapAttrs;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    attrs: MapAttrs
}

impl Default for Map {
    fn default() -> Self {
        Map::new(&mut MapAttrs::default())
    }
}

impl Map {
    pub fn new(map_attributes: &mut MapAttrs) -> Map {
        
        let base_height = &map_attributes.get_base_tile_height().to_owned();
        let width = map_attributes.get_width().to_owned();
        let length = map_attributes.get_length().to_owned();

        let map_tiles = vec![vec![Tile::new(&base_height); width]; length];
        
        Map {
            tiles: map_tiles,
            attrs: map_attributes.to_owned()
        }
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn get_mut_tiles(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.tiles
    }

    pub fn set_tiles(&mut self, map_tiles: Vec<Vec<Tile>>) {
        self.tiles = map_tiles;
    }

    pub fn get_tile(&self, row: usize, col: usize) -> &Tile {
        &self.tiles[row][col]
    }

    pub fn get_mut_tile(&mut self, row: usize, col: usize) -> &mut Tile {
        &mut self.tiles[row][col]
    }

    //this will be used when needing to completely change a tile, rather than any time any
    //of one's data changes
    pub fn set_tile(&mut self, a_tile: Tile, row: usize, col: usize) {
        self.tiles[row][col] = a_tile
    }

    pub fn get_attrs(&mut self) -> &MapAttrs {
        &self.attrs
    }
}