#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;

#[test]
fn test_generate_map() {
    let mut a_map: Map = Default::default();

    MapGenerator::generate_map(&mut a_map);
    for row in a_map.get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}

#[test]
fn test_placeholder_generator() {
    let mut a_map: Map = Default::default();

    MapGenerator::placeholder_generator(&mut a_map);
    for row in a_map.get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}