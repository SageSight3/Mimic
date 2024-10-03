use crate::modules::tile;
use crate::modules::map_attrs;

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Vec<tile::Tile>>
}

impl Map {
    pub fn new(map_attributes: &map_attrs::MapAttrs) -> Map {
        let tiles = vec![vec![tile::Tile::new(&map_attributes.base_tile_height); map_attributes.width]; map_attributes.length];
        Map {
            map: tiles
        }
    }
}