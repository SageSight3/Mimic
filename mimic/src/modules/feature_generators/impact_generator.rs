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
    //maybe move amount to throw rng to map generator -> would remove an extra arg pass to ImpactGenerator
    fn generate(a_map: &mut Map, frequency: &Distribution, depth_range: &Distribution) {
        //initialize ImpactGenerator

        //loop for number of impacts to generate
            //generate crater
        
        //place_undistributed_material()
    }

    pub fn generate_crater(a_map: &mut Map, crater_depth: u16, impact_coord: Coordinate) {
        //calculate radius
        //...
    }

    //set tile in the crater's height to height of the tile at the impact coordinate
    pub fn level_tile(a_map: &mut Map, tile_coord: Coordinate) {

    }

    //set tile in the crater's height to the right depth based off distance from impact coordinate and 
    //crater depth
    pub fn set_tile_depth(a_map: &mut Map, dist_from_center: u16, crater_depth: u16, tile_coord: Coordinate) {

    }

    //maybe unnecessary
    pub fn build_crater_lip(a_map: &mut Map, crater_radius: u16, crater_depth: u16, a_coord: Coordinate) {

    }

    //for redistributing eroded material back around the map
    //for Mimic V1, material just means units of height 
    pub fn place_undsitributed_material(a_map: &mut Map) {

    }

    //maybe unnecessary
    pub fn calculate_undistributed_material(a_map: &mut Map) {

    }
}