use crate::modules::map::Map;
use rand::Rng;

/*******************************************************
VERY WIP
*******************************************************/

#[derive(Debug)]
pub struct MapGenerator {
    duration: i32
    //generatorSpecifications
}

impl MapGenerator {
    pub fn setup() { //for generator specs

    }

    pub fn generate_map(a_map: &mut Map) { //Design WIP
        Self::setup();
        let instructions = MapGenerator { duration: 1 };
        for pass in 0..instructions.duration {
            Self::placeholder_generator(a_map)
        }
    }

    pub fn placeholder_generator(a_map: &mut Map) {
        for row in &mut a_map.tiles {
            for tile in row {
                //temporary, just to create something with the map
                let a_height: i32 = rand::thread_rng().gen_range(1..256);
                tile.set_height(a_height);
            }
        }
    }
}
