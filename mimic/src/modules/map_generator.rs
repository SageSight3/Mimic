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
        let instructions = MapGenerator { 
            duration: 1      
        };

        //temp until better implementation/specification implementation
        //impact generation
        let frequency: Distribution = Distribution::new(150, 600, 75);
        let depth_range: Distribution = Distribution::new(3, 75, -74);
        let impact_generator_delegate: ImpactGeneratorDelegate = ImpactGeneratorDelegate::new(
            &frequency, 
            &depth_range
        );

        let water_map_delegate: WaterMapDelegate = WaterMapDelegate::new();

        //water map generation

        for pass in 0..instructions.duration {

            //run generators that alter height map first
            //MapGenerator::placeholder_generator(a_map);
            impact_generator_delegate.run_pass(a_map);

            //non height map generators
            water_map_delegate.run_pass(a_map);
        }
    }

    pub fn placeholder_generator(a_map: &mut Map) {
        a_map.update_tiles(|a_tile: &mut Tile| {
            //temporary, just to create something with the map
            let a_height: i32 = rand::thread_rng().gen_range(1..256);
            a_tile.set_height(a_height);
        });
        a_map.compute_height_data();
    }
}

pub struct ImpactGeneratorDelegate {
    frequency: Distribution,
    depth_range: Distribution,
}

impl ImpactGeneratorDelegate {
    pub fn new(a_frequency: &Distribution, a_depth_range: &Distribution) -> ImpactGeneratorDelegate {
        ImpactGeneratorDelegate {
            frequency: a_frequency.clone(),
            depth_range: a_depth_range.clone()
        }
    }

    pub fn run_pass(&self, a_map: &mut Map) {
        let num_of_impacts: u16 = self.frequency.rand();
        ImpactGenerator::generate(a_map, num_of_impacts, &self.depth_range);
        a_map.compute_height_data(); //recalculate a_map's height data after impacts
    }
}

pub struct WaterMapDelegate {

}

impl WaterMapDelegate {
    pub fn new() -> WaterMapDelegate {
        WaterMapDelegate {

        }
    }

    pub fn run_pass(&self, a_map: &mut Map) {
        WaterMapGenerator::generate(a_map);
    }
}
