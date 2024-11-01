use eframe::egui::debug_text::print;
use rand::thread_rng;

use crate::modules::feature_generators::utility::Distribution;
use crate::modules::map::Map;
use crate::modules::map::Coordinate;
use crate::modules::tile;
use crate::modules::tile::Tile;
use rand::Rng;

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
    pub fn generate(a_map: &mut Map, num_of_impacts: u16, depth_range: &Distribution) {
        let mut impact_generator: ImpactGenerator = Default::default();

        //generate impacts
        for impact in 0..num_of_impacts {
            let impact_x: usize = thread_rng().gen_range(0..*a_map.get_width());
            let impact_y: usize = thread_rng().gen_range(0..*a_map.get_length());

            let an_impact_coord: Coordinate = Coordinate::new(impact_x, impact_y);

            let a_crater_depth: u16 = depth_range.rand();

            impact_generator.generate_crater(a_map, a_crater_depth, an_impact_coord);
        }

        impact_generator.place_undistributed_material(a_map);
    }

    //write tests for big, medium, and small craters, big = 167, small = 6, medium = 15
    pub fn generate_crater(&mut self, a_map: &mut Map, crater_depth: u16, impact_coord: Coordinate) {

        //compute crater info
        let variance: f32 = rand::thread_rng().gen_range(3.0..=4.0);

        let transient_radius: f32 = (variance * (crater_depth as f32)) / 3.0;
        let rim_radius: f32 = (transient_radius * (1.3 + rand::thread_rng().gen_range(0.0..=0.7))).round();

        //convert transient and rim radii to u16s
        let transient_radius: u16 = transient_radius.round() as u16;
        let rim_radius: u16 = rim_radius.round() as u16;

        let impact_x: usize = *impact_coord.get_X(); let impact_y: usize = *impact_coord.get_Y();
        let impact_coord_height: u16 = *a_map.get_tile(impact_y, impact_x).get_height() as u16;

        let tiles_coords: Vec<Vec<Coordinate>> = Self::crater_tiles_coords(a_map, rim_radius, &impact_coord);

        //generate crater
        let mut a_crater: Crater = Crater::new(
            variance,
            transient_radius,
            rim_radius,
            impact_coord_height,
            crater_depth,
            tiles_coords
        );

        a_crater.dig_transient_crater(a_map);
        a_crater.build_crater_rim(a_map);

        self.undistributed_height += a_crater.get_ejecta_volume();
    }

    /*for redistributing eroded material back around the map for Mimic V1, material just means units of height 
    for initial minimum viable delivery will just distribute material uniformly across map unless tile height was
    changed to be represented by a float, material redistribution may only have the intended effect if opting for 
    target number of total passes for full map generation to be 900 */
    pub fn place_undistributed_material(&mut self, a_map: &mut Map) {
        let height_to_add: i32 = (self.undistributed_height as f32 / (*a_map.get_length() * *a_map.get_width()) as f32).round() as i32;

        if height_to_add > 0 {
            a_map.update_tiles(|a_tile: &mut Tile| { a_tile.add_height(height_to_add); });
        }
    }

    pub fn get_undistributed_height(&self) -> &u32 {
        &self.undistributed_height
    } 
    
    pub fn set_undistributed_height(&mut self, height: u32) {
        self.undistributed_height = height;
    }

    pub fn crater_tiles_coords(a_map: &mut Map, rim_rad: u16, impact_coord: &Coordinate) -> Vec<Vec<Coordinate>> {
        let mut coords: Vec<Vec<Coordinate>> = Vec::new();
        for distance in 0..=rim_rad {
            coords.push(Vec::new());
        }

        let rad_int: i16 = rim_rad as i16;

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
                    
                    coords[dist_from_center.floor() as usize].push(Coordinate::new(x_coord as usize, y_coord as usize));
                }
            }
        }
        coords
    }
}

pub struct Crater {
    //a value between 3 and 4, inclusively, to vary transient radius between craters of equal max crater depth, is also needed to be
    //accounted for in depth at distance, x, calculation for transient crater
    variance: f32, 
    transient_radius: u16,
    rim_radius: u16,
    impact_coord_height: u16,
    crater_depth: u16,
    //visible_depth: u16,
    tiles_coords: Vec<Vec<Coordinate>>,
    ejecta_volume: u32 //units of height removed from crater
}

impl Crater {
    pub fn new(vari: f32, trans_rad: u16, rim_rad: u16, impact_height: u16, depth: u16, coords: Vec<Vec<Coordinate>>) -> Crater {
        Crater {
            variance: vari,
            transient_radius: trans_rad,
            rim_radius: rim_rad,
            impact_coord_height: impact_height,
            crater_depth: depth,
            //visible_depth: depth*3, //based off research, crater depth we can see is usually only around 1/3 of the crater's total depth
            tiles_coords: coords,
            ejecta_volume: 0 as u32
        }
    }

    //changes heights of all tiles in transient crater, using a formula based on research
    //see docs/DesignNotes/revised_trans_crater_math.png for more info
    pub fn dig_transient_crater(&mut self, a_map: &mut Map) {

        //var names from ax^2 - c
        let ax_dividend: f32 = (self.crater_depth as f32) * (self.variance.powi(2));
        let c: f32 = self.crater_depth as f32;

        for (dist_from_center, coords_at_dist) in self.tiles_coords.iter().enumerate() {
            if dist_from_center as u16 > self.transient_radius {
                break;
            }

            //find height at dist_from_center from crater center
            let ax_divisor: f32 = 9.0 * (dist_from_center as f32).powi(2);
            let height_diff: f32 =  ((ax_divisor/ax_dividend) - c).round();
            //add height diff to impact_coord_height (should be <= 0)
            let height: u16 = ((self.impact_coord_height as f32 + height_diff).floor()) as u16;

            for coord in coords_at_dist {
                let mut a_tile = a_map.get_mut_tile(*coord.get_Y(), *coord.get_X());
                if *a_tile.get_height() > height as i32 {
                    let old_height: u32 = *a_tile.get_height() as u32;
                    if old_height as i32 == i32::MAX {
                        println!("invalid tile height: {}", *a_tile.get_height());
                    }
                    a_tile.set_height(height as i32);

                    //add carved out volume to ejecta for post-crater-gen redstribution
                    if (u32::MAX - old_height - height as u32) <= self.ejecta_volume {
                        println!("ejecta_volume limit reached");
                        println!("ejecta_volume: {}", self.ejecta_volume);
                        println!("new ejecta: {}", old_height - height as u32);
                        println!("old_height: {}", old_height);
                        println!("new height: {}", height);
                        println!("height_diff: {}", height_diff);
                        println!("impact_coord_height: {}", self.impact_coord_height);
                    }
                    self.ejecta_volume += (old_height - height as u32);
                    
                }
            }
        }
        /*
        if self.crater_depth > 50 {
            println!("ejecta_volume: {}", self.ejecta_volume);
        }
        */
    }

    pub fn build_crater_rim(&mut self, a_map: &mut Map) {
        let max_added_height: f32 = (0.1) * (2.0 * self.transient_radius as f32);
        let max_height_dist: u16 = self.build_rim_upward_slope(a_map, max_added_height);
        self.build_rim_downward_slope(a_map, max_added_height, max_height_dist);
    }

    /* adds height to tiles still following transient crater paraboloid from transient radius, to 
    distance of max added height ***Note***: since the heights of tiles in this range still follow the 
    shape of the transient crater, their height will be reflect the impact_coordinate's pre-crater 
    surface height over their own. Function in many senses, mimics build_transient_crater() Look into 
    refactoring in future, and stopping */
    fn build_rim_upward_slope(&mut self, a_map: &mut Map, max_added_height: f32) -> u16 {
        //while for building the rim's upward slope, max_added_height needs to be rounded to return
        //the most accurate distance that it occurs at, the downward slope needs it for it's formula, so
        //this function will just create a rounded copy locally
        let rounded_max: f32 = max_added_height.round();

        //height change formula initial setup, var names from ax^2 - c
        let ax_dividend: f32 = (self.crater_depth as f32) * (self.variance.powi(2));
        let c: f32 = self.crater_depth as f32;

        //just using a range here, since not iterating over tiles_coords from the beginning
        for dist_from_center in (self.transient_radius + 1)..=self.rim_radius {
            //find height at dist_from_center from crater center
            let ax_divisor: f32 = 9.0 * (dist_from_center as f32).powi(2);
            let height_to_add: f32 =  ((ax_divisor/ax_dividend) - c).round(); //should be >= 0

            //if height_to_add exceeds rounded_max, we know that max_added_height occurred
            //at last distance checked, aka last distance checked was top of upward slope
            if height_to_add > rounded_max {
                return dist_from_center - 1;
            }

            let new_height: u16 = ((self.impact_coord_height as f32 + height_to_add).floor()) as u16;

            for coord in self.tiles_coords[dist_from_center as usize].clone() {
                let mut a_tile = a_map.get_mut_tile(*coord.get_Y(), *coord.get_X());
                if new_height < *a_tile.get_height() as u16 {
                    let old_height: u32 = a_tile.get_height().clone() as u32;
                    a_tile.set_height(new_height as i32);

                    //add carved out volume to ejecta for post-crater-gen redstribution
                    //unlike transient crater, new height will be bigger than old height
                    self.ejecta_volume += old_height - new_height as u32; 
                }
            }
        }
        //if edge of transient crater is edge of upward slope, return transient radius as max_height_dist
        self.transient_radius
    }

    //adds height to tiles sloping downwards from the distance of the maximum added height to the crater
    //rim's edge
    fn build_rim_downward_slope(&mut self, a_map: &mut Map, max_added_height: f32, max_height_dist: u16) {

        //refer to rim_height_math.png for formula and name definitions
        let md: u16 = max_height_dist - self.transient_radius;
        let dividend: f32 = (self.transient_radius as f32 + md as f32);

        for dist_from_center in (max_height_dist + 1)..=self.rim_radius {
            let divisor: f32 = max_added_height * (dist_from_center as f32 - (self.transient_radius as f32 - md as f32));

            let height_to_add: u16 = (max_added_height as f32 - (divisor/dividend)).round() as u16;
            for coord in self.tiles_coords[dist_from_center as usize].clone() {
                let mut a_tile = a_map.get_mut_tile(*coord.get_Y(), *coord.get_X());

                let new_height: i32 = *a_tile.get_height() + height_to_add as i32;

                //add height_to_add to a tile's current height
                a_tile.set_height(new_height);
            }
        }
    }

    pub fn get_variance(&self) -> &f32 {
        &self.variance
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