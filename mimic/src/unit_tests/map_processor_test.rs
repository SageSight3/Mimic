#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;
use crate::modules::map_processor::MapProcessor;

#[test]
fn test_generate_map() {
    let mut a_map_processor: MapProcessor = Default::default();

    a_map_processor.get_mut_map().generate_map();
    for row in a_map_processor.get_map().get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}