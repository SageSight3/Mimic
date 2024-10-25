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

        let rad_int: i16 = total_rad as i16;

        let mut tile_count: i32 = 0;

        for y in (rad_int*-1)..=rad_int {
            for x in (rad_int*-1)..=rad_int {

                let dist_from_center: f32 = (((x as f32).powi(2) + (y as f32).powi(2))).sqrt();
                if dist_from_center <= rad_int as f32 {
                    
                    let mut x_coord: i16 = *impact_coord.get_X() as i16 + x;
                    let mut y_coord: i16  = *impact_coord.get_Y() as i16 + y;

                    /****************************************************************
                      * if exceeding bounds, wrap crater around to other side of map
                    *****************************************************************/

                    if x_coord >= *a_map.get_width() as i16 {
                        x_coord -= *a_map.get_width() as i16;
                    }

                    if x_coord < 0 {
                        x_coord += *a_map.get_width() as i16;
                    }

                    if y_coord >= *a_map.get_length() as i16 {
                        y_coord -= *a_map.get_length() as i16;
                    }
                    
                    if y_coord < 0 {
                        y_coord += *a_map.get_length() as i16;
                    }

                    /*debug
                    if y_coord == 400{
                        println!("{}, {}, {}", dist_from_center as i32, x_coord, y_coord);
                    }
                    */
                    
                    coords[dist_from_center.floor() as usize].push(Coordinate::new(x_coord as usize, y_coord as usize));
                }
            }
        }
        coords
    }

    pub fn calculate_transient_radius() {

    }

    pub fn calculate_rim_radius() {

    }
}

pub struct Crater {
    transient_radius: u16,
    rim_radius: u16,
    impact_coord_height: u16,
    crater_depth: u16,
    //visible_depth: u16,
    tiles_coords: Vec<Vec<Coordinate>>,
    ejecta_volume: u32 //units of height removed from crater
}

impl Crater {
    pub fn new(trans_rad: u16, rim_rad: u16, impact_height: u16, depth: u16, coords: Vec<Vec<Coordinate>>) -> Crater {
        Crater {
            transient_radius: trans_rad,
            rim_radius: rim_rad,
            impact_coord_height: impact_height,
            crater_depth: depth,
            //visible_depth: depth*3, //based off research, crater depth we can see is usually only around 1/3 of the crater's total depth
            tiles_coords: coords,
            ejecta_volume: 0 as u32
        }
    }

    pub fn dig_transient_crater(&mut self, a_map: &mut Map) {

        //for finding height at distance, d, from center of crater
        let mut trans_crater_height_range: Vec<u16> = Vec::new();
        //var names from ax^2 - c
        let ax_dividend: f32 = /*3.0 * */ (self.transient_radius as f32).powi(2);
        //3.0 from research that excavation depthing is usually only about a third of total crater depth
        let c: f32 = self.crater_depth as f32;

        for (dist_from_center, coords_at_dist) in self.tiles_coords.iter().enumerate() {
            if dist_from_center as u16 > self.transient_radius {
                break;
            }
            //find height at dist_from_center from crater center
            let ax_divisor: f32 = (self.crater_depth as f32) * (dist_from_center as f32).powi(2);
            let height_diff: f32 =  (ax_divisor/ax_dividend) - c;
            let height: u16 = ((self.impact_coord_height as f32 + height_diff).floor()) as u16;

            println!("height: {}, dist: {}", height, dist_from_center);
            for coord in coords_at_dist {
                let mut a_tile = a_map.get_mut_tile(*coord.get_X(), *coord.get_Y());
                if *a_tile.get_height() as u16 > height {
                    let old_height: u32 = a_tile.get_height().clone() as u32;
                    //println!("old_tile_height: {}", *a_tile.get_height());
                    a_tile.set_height(height as i32);
                    //println!("tile_height: {}", *a_tile.get_height());
                    self.ejecta_volume += (old_height - height as u32);
                }
            }
        }
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

    pub fn get_impact_height(&self) -> &u16 {
        &self.impact_coord_height
    }

    pub fn get_crater_depth(&self) -> &u16 {
        &self.crater_depth
    }

    pub fn get_tiles_coords(&self) -> &Vec<Vec<Coordinate>> {
        &self.tiles_coords
    }

    pub fn get_ejecta_volume(&self) -> &u32 {
        &self.ejecta_volume
    }
}