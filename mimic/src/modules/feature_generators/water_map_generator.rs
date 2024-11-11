use crate::modules::map::Map;
use crate::modules::tile::Tile;

//Subject to change
#[derive(Debug, Clone)]
pub struct WaterMapGenerator {
}

impl WaterMapGenerator {
    pub fn new() -> WaterMapGenerator {
        WaterMapGenerator {
        }
    }

    //look into refactoring to return a &u64 instead of u64
    pub fn generate(a_map: &mut Map) {
        let mut water_map_generator: WaterMapGenerator = WaterMapGenerator::new();

        let sea_level: i32 = water_map_generator.find_sea_level(a_map);
        a_map.set_sea_level(sea_level);

        a_map.update_tiles(|tile: &mut Tile| {
            if *tile.get_height() <= sea_level && !(*tile.has_water()) {
                tile.add_water();
            } else if *tile.get_height() > sea_level && *tile.has_water() { //in case water from a previous pass needs to be removed
                tile.remove_water();
            }
        });
    }

    //binary search height range for height where tiles at or below it = target_tile_num
    pub fn find_sea_level(&self, a_map: &Map) -> i32 { 

        //To minimizae execution time, will only be sampling every fifth tile of every fifth row, so divide by 25
        let target_tile_num: i32 = *a_map.get_estimated_water_surface_area() as i32/25;

        let mut sea_level: i32 = i32::MAX;
        let mut tiles_at_current_height: i32 = 0;
        let mut tiles_at_prev_height: i32 = 0;
        let mut low: i32 = *a_map.get_min_height();
        let mut high: i32 = *a_map.get_max_height();
        
        //Because the chances of finding an exact match are low, we'll remember our previous tested heights, so later, 
        //we can return the closer of the two heights with the closest number of tiles to target_tile_num
        let mut current_height_to_test: i32 = i32::MAX;
        let mut prev_height_to_test: i32 = i32::MAX;

        while low <= high {
            current_height_to_test = low + (high - low) / 2;
            let mut count: i32 = 0;

            //to sample 20% of the map, only sample every 5th tile in every 5th row row and 5th tile
            for row in a_map.get_tiles().iter().step_by(5) {
                let filtered_row: Vec<&Tile> = row.iter().step_by(5)
                    .filter(|tile| *tile.get_height() <= current_height_to_test)
                    .collect::<Vec<&Tile>>();

                count += filtered_row.len() as i32;
            }

            tiles_at_current_height = count;

            if tiles_at_current_height == target_tile_num { //on the off chance of finding an exact match
                sea_level = current_height_to_test;
                return sea_level;
            } else if tiles_at_current_height > target_tile_num {
                tiles_at_prev_height = tiles_at_current_height;
                prev_height_to_test = current_height_to_test;
                high = current_height_to_test - 1;
            } else {
                tiles_at_prev_height = tiles_at_current_height;
                prev_height_to_test = current_height_to_test;
                low = current_height_to_test + 1;
            }
        }

        //find height with closest number of tiles to target_tile_num
        let prev_proximity: i32 = (target_tile_num - tiles_at_prev_height).abs();
        let current_proximity: i32 = (target_tile_num - tiles_at_current_height).abs();

        if current_proximity < prev_proximity {
            sea_level = current_height_to_test;
        } else {
            sea_level = prev_height_to_test;
        }

        sea_level
    }
}