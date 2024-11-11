use eframe::egui::warn_if_debug_build;

#[cfg(test)]
use super::*;
use crate::modules::map::Map;
use crate::modules::setup;
use crate::modules::feature_generators::water_map_generator::WaterMapGenerator;

#[test]
fn test_find_sea_level() {
    let mut a_map: Map = Default::default();
    let base_height: i32 = 200;
    setup::set_up_map(&mut a_map, base_height);

    let water_map_generator: WaterMapGenerator = WaterMapGenerator::new();

    let sea_level: i32 = water_map_generator.find_sea_level(&mut a_map);

    assert!(sea_level != i32::MAX);
}

#[test]
fn test_generate() {
    let mut a_map: Map = Default::default();
    let base_height: i32 = 200;
    setup::set_up_map(&mut a_map, base_height);

    WaterMapGenerator::generate(&mut a_map);

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