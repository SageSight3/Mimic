#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;
use crate::modules::map_processor::MapProcessor;

#[test]
fn test_new() {
    let a_map: Map = Default::default();
    let a_map_processor = MapProcessor::new(a_map.clone());

    let processor_map_tiles = a_map_processor.get_map().get_tiles();
    assert_eq!(a_map_processor.get_map().get_tiles().len(), a_map.get_tiles().len());
}

#[test]
fn test_generate_map() {
    let mut a_map_processor: MapProcessor = Default::default();

    a_map_processor.generate_map();
    for row in a_map_processor.get_map().get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}

#[test]
fn test_process_map() {
    let mut a_map_processor: MapProcessor = Default::default();

    a_map_processor.process_map();
    for row in a_map_processor.get_map().get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}

#[test]
fn test_set_map() {
    let mut a_map_processor: MapProcessor = Default::default();
    let default_map: Map = Default::default();
    let a_map: Map = Map::new(&mut MapAttrs::new(100, 400, 3));

    assert_eq!(a_map_processor.get_map().get_tiles().len(), default_map.get_tiles().len());

    a_map_processor.set_map(a_map.clone());

    assert_eq!(a_map_processor.get_map().get_tiles().len(), a_map.get_tiles().len());
}