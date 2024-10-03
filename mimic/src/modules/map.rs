use crate::modules::tile::Tile;

const WIDTH: usize = 414;
const LENGTH: usize = 414;

pub struct Map {
    pub height_map: [[Tile; WIDTH]; LENGTH]
}

impl Default for Map {
    fn default() -> Self {
        Self {
            height_map: build_empty_map()
        }
    }
}


fn build_empty_map() -> [[Tile; WIDTH]; LENGTH] {
    let mut empty_map = [[Tile { ..Default::default() }; WIDTH]; LENGTH];
    empty_map
}

