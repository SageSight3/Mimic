use crate::modules::tile::Tile;

pub const WIDTH: usize = 414;
pub const LENGTH: usize = 414;

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
    let empty_map = [[Tile { ..Default::default() }; WIDTH]; LENGTH];
    empty_map
}

