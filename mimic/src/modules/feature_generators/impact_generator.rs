use crate::modules::feature_generators::utility::Distribution;
use crate::modules::map::Map;
use crate::modules::map::Coordinate;

#[derive(Debug, Clone)]
pub struct ImpactGenerator {
    undistributed_height: u32
}

impl Default for ImpactGenerator {
    fn default() -> Self {
        Self {
            undistributed_height: 0
        }
    }
}

impl ImpactGenerator {
    fn generate(a_map: &mut Map, frequency: &Distribution, depth_range: &Distribution) {

    }

    pub fn generate_crater(a_map: &mut Map, crater_depth: u16, a_coord: Coordinate) {

    }

    pub fn level_crater_site(a_map: &mut Map, crater_radius: u16, a_coord: Coordinate) {

    }

    pub fn dig_crater(a_map: &mut Map, crater_radius: u16, crater_depth: u16, a_coord: Coordinate) {

    }

    pub fn build_crater_lip(a_map: &mut Map, crater_radius: u16, crater_depth: u16, a_coord: Coordinate) {

    }

    pub fn place_undsitributed_height(a_map: &mut Map) {

    }

    pub fn calculate_undistributed_height(a_map: &mut Map) {

    }
}