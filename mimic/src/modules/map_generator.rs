use crate::modules::map::Map;
use rand::Rng;
use crate::modules::feature_generators::impact_generator::ImpactGenerator;

/*******************************************************
VERY WIP
*******************************************************/

#[derive(Debug)]
pub struct MapGenerator {
    duration: i32
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
        for row in a_map.get_mut_tiles() {
            for tile in row {
                //temporary, just to create something with the map
                let a_height: i32 = rand::thread_rng().gen_range(1..256);
                tile.set_height(a_height);
            }
        }
    }
}
