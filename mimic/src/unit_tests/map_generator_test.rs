#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;
use crate::modules::feature_generators::utility::Distribution;
use crate::modules::feature_generators::impact_generator::ImpactGenerator;
use crate::modules::feature_generators::water_map_generator::WaterMapGenerator;
use crate::modules::setup;
use crate::modules::map_generator::ImpactGeneratorDelegate;
use crate::modules::map_generator::WaterMapDelegate;

#[test]
fn test_generate_map() {
    let base_height: i32 = 200;
    let mut a_map: Map = Default::default();
    a_map.update_tiles(| a_tile: &mut Tile | { 
        a_tile.set_height(base_height);
    });

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 200);
    });
    
    MapGenerator::generate_map(&mut a_map);

    //test height map generation
    let mut generated: bool = false;
    for row in a_map.get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            if *col.get_height() != base_height {
                generated = true;
            }
        }
    }
    assert!(generated);

    //test water map genereration
    assert!(*a_map.get_sea_level() != i32::MAX);
    let mut found_water: bool = true;
    for row in a_map.get_tiles() {
        for tile in row {
            if *tile.has_water() {
                found_water = true;
            }
        }
    }
    assert!(found_water);
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

    assert!(*a_map.get_height_range() > 0);
}

//Still need to update generate_map() test
#[test]
fn test_impact_generator_delegate() {
    let base_height: i32 = 200;
    let mut a_map: Map = Default::default();
    a_map.update_tiles(| a_tile: &mut Tile | { 
        a_tile.set_height(base_height);
    });

    let frequency: Distribution = Distribution::new(45, 90, 23);
    let depth_range: Distribution = Distribution::new(1, 30, 12);
    let an_impact_generator_delegate: ImpactGeneratorDelegate = ImpactGeneratorDelegate::new(
        &frequency, 
        &depth_range
    );

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 200);
    });

    an_impact_generator_delegate.run_pass(&mut a_map);

    let mut generated: bool = false;
    for row in a_map.get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            if *col.get_height() != base_height {
                generated = true;
            }
        }
    }
    assert!(generated);
    assert!(*a_map.get_height_range() > 0);
}

//Still need to update generate_map() test
#[test]
fn test_water_map_generator() {
    let mut a_map: Map = Default::default();
    let base_height: i32 = 200;
    setup::set_up_map(&mut a_map, base_height);

    let a_water_map_delegate: WaterMapDelegate = WaterMapDelegate::new();
    a_water_map_delegate.run_pass(&mut a_map);

    assert!(*a_map.get_sea_level() != i32::MAX);
    
    let mut found_water: bool = true;
    for row in a_map.get_tiles() {
        for tile in row {
            if *tile.has_water() {
                found_water = true;
            }
        }
    }

    assert!(found_water);
}