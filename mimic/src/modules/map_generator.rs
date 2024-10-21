use crate::modules::map::Map;
use rand::Rng;
use crate::modules::feature_generators::impact_generator::ImpactGenerator;
use crate::modules::feature_generators::water_map_generator::WaterMapGenerator;
use crate::modules::feature_generators::utility::Distribution;
//remove once removing placeholder generator
use crate::modules::tile::Tile;

/*******************************************************
A MapGenerator will be responsible for managing Mimic's
feature generators, with future plans, to increase the
flexibility of that management via implementation
of a map's specification
*******************************************************/

#[derive(Debug)]
pub struct MapGenerator {
    duration: i32,
    //generatorSpecifications
}

impl MapGenerator {

    pub fn generate_map(a_map: &mut Map) { //Design WIP
        let instructions = MapGenerator { duration: 1 };
        for pass in 0..instructions.duration {
            Self::placeholder_generator(a_map)
        }
    }

    pub fn placeholder_generator(a_map: &mut Map) {
        a_map.update_tiles(|a_tile: &mut Tile| {
            //temporary, just to create something with the map
            let a_height: i32 = rand::thread_rng().gen_range(1..256);
            a_tile.set_height(a_height);
        });
    }

    pub fn impact_generator(a_map: &mut Map, frequency: &Distribution, depth_range: &Distribution) {

    }

    pub fn water_map_generator(a_map: &mut Map, percent_volume: u8) {

    }
}
