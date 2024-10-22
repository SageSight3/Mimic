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
    pub fn generate_crater(&mut self, a_map: &mut Map, crater_depth: u16, impact_coord: Coordinate) {
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
    pub fn place_undsitributed_material(&mut self, a_map: &mut Map) {
        //height_to_add = (undristributed_material/(a_map.get_length() * a_map.get_width())) as i32
        //a_map.update_tiles(|a_tile: &mut Tile| { a_tile.add_height(height_to_add); });
    }

    pub fn get_undistributed_height(&self) -> &u32 {
        &self.undistributed_height
    } 

    pub fn crater_tiles_coords(a_map: &mut Map, total_rad: u16, impact_coord: &Coordinate) -> Vec<Vec<Coordinate>> {
        let mut coords: Vec<Vec<Coordinate>> = Vec::new();
        for distance in 0..=total_rad {
            coords.push(Vec::new());
        }

        let start_y_index: usize;
        let start_x_index: usize;
        let end_y_index: usize;
        let end_x_index: usize;

        if *impact_coord.get_Y() < total_rad as usize { //if start_y_index would be less than 0
            start_y_index = *a_map.get_length() - 1 - (total_rad as usize - *impact_coord.get_Y());
        } else {
            start_y_index = *impact_coord.get_Y() - total_rad as usize;
        }

        if *impact_coord.get_X() < total_rad as usize { //if start_x_index would be less than 0
            start_x_index = *a_map.get_width() - 1 - (total_rad as usize - *impact_coord.get_X());
        } else {
            start_x_index = *impact_coord.get_X() - total_rad as usize;
        }

        //if end_y_index would be greater than a_map.length()
        if (start_y_index + ((2*total_rad) as usize)) > *a_map.get_length() {
            end_y_index = 0 + ((start_y_index + ((2*total_rad) as usize)) - *a_map.get_length());
        } else {
            end_y_index = start_y_index + ((2 * total_rad) as usize);
        }

        //if end_x_index would be greater than a_map.width()
        if (start_x_index + ((2*total_rad) as usize)) > *a_map.get_width() {
            end_x_index = 0 + ((start_x_index + ((2*total_rad) as usize)) - *a_map.get_width());
        } else {
            end_x_index = start_x_index + ((2 * total_rad) as usize);
        }

        //scan every coordinate in square where length = total_crater_diameter, beginning from 
        //the starting indices
        for mut y_index in start_y_index..=end_y_index {
            if y_index >= *a_map.get_length() {
                y_index = 0 + y_index - *a_map.get_length();
            }
            for mut x_index in start_x_index..=end_x_index {
                if x_index >= *a_map.get_width() {
                    x_index = 0 + x_index - *a_map.get_width();
                }

                //coord distance from impact coord
                let delta_x: f32 = (*impact_coord.get_X() - x_index) as f32;
                let delta_y: f32 = (*impact_coord.get_Y() - y_index) as f32;
                let dist_from_center: usize = (delta_x.powi(2) + delta_y.powi(2)).sqrt() as usize;

                if dist_from_center as u16 <= total_rad {
                    coords[dist_from_center].push(Coordinate::new(x_index, y_index));
                }
            }
        }

        coords
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