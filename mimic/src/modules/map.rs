use std::collections::binary_heap;

use crate::modules::tile::Tile;
use crate::modules::map_attrs::MapAttrs;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    attrs: MapAttrs,

    //water_data
    water_percent_surface_area: f32, //percent of surface under water
    estimated_water_surface_area: u32, //esitimated number of tiles that should be under water
    sea_level: i32,

    //height data
    height_range: i32,
    min_height: i32,
    max_height: i32,
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
        
        let mut a_map = Map {
            tiles: map_tiles,
            attrs: map_attributes.to_owned(),
            water_percent_surface_area: 0.5,
            estimated_water_surface_area: 0,
            sea_level: i32::MAX,
            height_range: 0,
            min_height: 0,
            max_height: 0
        };

        a_map.compute_height_data();
        a_map.estimate_water_sureface_area();

        a_map
    }
    
    pub fn random_coordinate(&self) -> Coordinate {
        let x: usize = rand::thread_rng().gen_range(0..*self.attrs.get_width());
        let y: usize = rand::thread_rng().gen_range(0..*self.attrs.get_length());

        let a_coordinate: Coordinate = Coordinate::new(x, y);
        a_coordinate
    }

    pub fn update_tiles(&mut self, update_fn: impl Fn(&mut Tile)) { //update function/closure shouldn't take arguments
        for row in self.get_mut_tiles() {
            for tile in row {
                update_fn(tile);
            }
        }
    }

    //update tiles based on their position
    pub fn update_tiles_positionally(&mut self, mut update_fn: impl FnMut(&Coordinate, &mut Tile)) {
        for (y_coord, row) in self.tiles.iter_mut().enumerate() {
            for (x_coord, tile) in row.iter_mut().enumerate() {
                let coord = Coordinate::new(x_coord, y_coord);
                update_fn(&coord, tile)
            }
        }
    }

    pub fn compute_height_data(&mut self) { //should be re-computed after anything that changes tile height in the map
        let mut min: i64 = i64::MAX;
        let mut max: i64 = i64::MIN;

        for row in &self.tiles {
            for tile in row  {
                let a_height: i32 = (*tile.get_height());

                if (a_height as i64) < min { min = a_height as i64; }
                if (a_height as i64) > max { max = a_height as i64; }
            }
        }

        self.min_height = min as i32;
        self.max_height = max as i32;
        self.height_range = (max - min) as i32;
    }

    pub fn estimate_water_sureface_area(&mut self) {
        let total_tiles: u32 = (*self.attrs.get_length() * *self.attrs.get_width()) as u32;

        self.estimated_water_surface_area = (self.water_percent_surface_area * total_tiles as f32).round() as u32;
    }

    //getters and setters
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

    pub fn get_water_percent_surface_area(&self) -> &f32 {
        &self.water_percent_surface_area
    }

    pub fn get_estimated_water_surface_area(&self) -> &u32 {
        &self.estimated_water_surface_area
    }

    //this will be used when needing to completely change a tile, rather than any time any
    //of one's data changes
    pub fn set_tile(&mut self, a_tile: Tile, row: usize, col: usize) {
        self.tiles[row][col] = a_tile
    }

    pub fn get_attrs(&self) -> &MapAttrs {
        &self.attrs
    }

    pub fn get_length(&self) -> &usize {
        self.attrs.get_length()
    }

    pub fn get_width(&self) -> &usize {
        self.attrs.get_width()
    }

    pub fn get_height_range(&self) -> &i32 {
        &self.height_range
    }

    pub fn get_min_height(&self) -> &i32 {
        &self.min_height
    }

    pub fn get_max_height(&self) -> &i32 {
        &self.max_height
    }

    pub fn get_sea_level(&self) -> &i32 {
        &self.sea_level
    }

    pub fn set_sea_level(&mut self, a_height: i32) {
        self.sea_level = a_height;
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    pub fn new(x_coord: usize, y_coord: usize) -> Coordinate {
        Coordinate {
            x: x_coord,
            y: y_coord
        }
    }

    pub fn get_X(&self) -> &usize {
        &self.x
    }

    pub fn get_Y(&self) -> &usize {
        &self.y
    }
}