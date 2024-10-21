use crate::modules::map::Map;
use crate::modules::tile::Tile;

//Subject to change
#[derive(Debug, Clone)]
pub struct WaterMapGenerator {
    volume: u64
}

impl Default for WaterMapGenerator {
    fn default() -> Self {
        Self {
            volume: 0
        }
    }
}

impl WaterMapGenerator {

    //look into refactoring to return a &u64 instead of u64
    pub fn generate(a_map: &mut Map, percent_volume: u8) -> u64 {
        let mut water_map_generator: WaterMapGenerator = Default::default();
        //do stuff

        water_map_generator.volume
    }

    pub fn get_water_volume(&self) -> &u64 {
        &self.volume
    }
}