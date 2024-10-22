use crate::modules::feature_generators::utility::Distribution;
use crate::modules::map::Map;
use crate::modules::map::Coordinate;
use crate::modules::tile::Tile;

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

//Stretch goal: revise later to be able to make simple and complex craters and maybe crater basins

impl ImpactGenerator {
    //maybe move amount to throw rng to map generator -> would remove an extra arg pass to ImpactGenerator
    fn generate(a_map: &mut Map, frequency: &Distribution, depth_range: &Distribution) {
        //initialize ImpactGenerator

        //loop for number of impacts to generate
            //generate an impact coord
            //generate a crate_depth
            //generate crater
        
        //impact_generator.place_undistributed_material(a_map)
    }

    //write tests for big, medium, and small craters, big = 167, small = 6, medium = 15
    pub fn generate_crater(a_map: &mut Map, crater_depth: u16, impact_coord: Coordinate) {
        //calculate transient radius
        //calculate rim radius
        //look into: ((delta_x^2 + delta_y^2) <= radius^2) == (distance <= radius), both pow and sqrt are monotonic
        //create double vector, crater_tile_coords, of length radius where //outer vector should be a length of rim_radius+1
            //each row holds a vector of coordinates at 0..= radius distance from impact_coord, where radius = index
        //put info into Crater struct
        
        //dig_transient_crater
        //self.undistributed_height += crater.ejecta_volume
        //build crater rim
    }

    /*for redistributing eroded material back around the map for Mimic V1, material just means units of height 
    for initial minimum viable delivery will just distribute material uniformly across map unless tile height was
    changed to be represented by a float, material redistribution may only have the intended effect if opting for 
    target number of total passes for full map generation to be 900 */
    pub fn place_undsitributed_material(a_map: &mut Map) {
        //height_to_add = (undristributed_material/(a_map.get_length() * a_map.get_width())) as i32
        //a_map.update_tiles(|a_tile: &mut Tile| { a_tile.add_height(height_to_add); });
    }

    pub fn get_undistributed_height(&self) -> &u32 {
        &self.undistributed_height
    } 
}

pub struct Crater {
    transient_radius: u16,
    rim_radius: u16,
    crater_depth: u16,
    tile_coords: Vec<Vec<Coordinate>>,
    ejecta_volume: u32 //units of height removed from crater
}

impl Crater {
    pub fn new(trans_rad: u16, rim_rad: u16, depth: u16, coords: Vec<Vec<Coordinate>>) -> Crater {
        Crater {
            transient_radius: trans_rad,
            rim_radius: rim_rad,
            crater_depth: depth,
            tile_coords: coords,
            ejecta_volume: 0 as u32
        }
    }

    pub fn dig_transient_crater(&self, a_map: &mut Map) {
        //impact_coord_x = self.tile_coords[0][0].get_X()
        //impact_coord_y = self.tile_coords[0][0].get_Y()
        //impact_coord_height = a_map.get_tile(impact_coord_x, impact_coord_y).get_height()

        //loop through self.tile_coords from 0..=self.transient_radius,
            //calculate depth at radius = current_index
            //transient_ring = self.tile_coords[currentIndex]
            //loop through each coordinate in transient_ring
                //x = coord.get_X()
                //y = coord.get_Y()

                //old_tile_height = a_map.get_mut_tile(x, y).get_height()
                //new_height = impact_coord_height - depth
                
                //if old_tile_height < new_height
                    //continue
                //else
                    //a_map.get_mut_tile(x, y).set_height(new_height)
                    //ejecta_volume += ((2/3)*(old_tile_height - new_tile_height)) //2/3 based off impact crater research
  
        //change height and add ejecta from tile at center of crater
        //a_map.get_mut_tile(impact_coord_x, impact_coord_y).remove_height(self.max_depth)
        //ejecta_volume += ((2/3)*self.max_depth)
    }

    pub fn build_crater_rim(&self, a_map: &mut Map) {
        //loop through craterInfo.tile_coords from crater_info.transient_radius + 1 to craterInfo.tile_coords.len()
            //calculate height increase at radius = curent_index
            //rim_ring = craterInfo.tile_coords[currentIndex]
            //loop through coords in rim_ring
                //x = coord.get_X()
                //y = coord.get_Y()
                //map.get_mut_tile(x, y).add_height(height_increase)
    }

    pub fn get_transient_radius(&self) -> &u16 {
        &self.transient_radius
    }

    pub fn get_rim_radius(&self) -> &u16 {
        &self.rim_radius
    }

    pub fn get_crater_depth(&self) -> &u16 {
        &self.crater_depth
    }

    pub fn get_tile_coords(&self) -> &Vec<Vec<Coordinate>> {
        &self.tile_coords
    }

    pub fn get_ejecta_volume(&self) -> &u32 {
        &self.ejecta_volume
    }
}