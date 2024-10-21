#[cfg(test)]
use super::*;
use crate::modules::feature_generators::utility::Distribution;
use crate::modules::feature_generators::water_map_generator::WaterMapGenerator;

#[test]
fn test_default() {
    let water_map_generator: WaterMapGenerator = Default::default();

    assert_eq!(*water_map_generator.get_water_volume(), 0 as u64);
}

#[test]
fn test_generate() {
    assert!(false)
}