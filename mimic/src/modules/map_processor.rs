use crate::modules::app::Observer;
use crate::modules::app::GUI_OBSERVER;
use std::sync::OnceLock;
use crate::modules::map::Map;
use crate::modules::map_generator::MapGenerator;
use crate::modules::image_interpreter::ImageData;
use crate::modules::image_generator::ImageGenerator;
use crate::modules::tile::Tile; //may be temporary
use crate::modules::setup;
use std::io;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct MapProcessor {
    map: Map,
    map_name: String,
    status: String,
    //map_specification: MapSpecification,
    //generation_tasks: Vec<GenerationTask>,
    map_image_data: Option<ImageData>, //data to be passed to map/image interpreter, may not be necessary
    map_image_path: String
    //generated_map_image: Image, //look into how would be set up
    //state: //figure out
    //StateDescription: String //may not be used
}

impl Default for MapProcessor {
   fn default() -> Self {
        Self {
            map: Default::default(),
            map_name: "Branch-Main".to_string(),
            status: "Map generator ready!".to_string(),
            map_image_data: None,
            map_image_path: "".to_string()
        }
    }
}

impl MapProcessor {

    pub fn new(a_map: Map) -> MapProcessor {
        MapProcessor {
             map: a_map,
             ..Default::default()
        }
    }

    pub fn process_map(&mut self) {

        println!("\n***Mimic Procedural Map Generator***\n");
        
        //parse_specification(a_map_specification);

        //set map to a base height, may be moved later/may be defined by map specification
        /*
        let base_height: i32 = 200;
        self.map.update_tiles(|a_tile: &mut Tile| {
            a_tile.set_height(base_height);
        });
        */

        //noise generation
        let base_height: i32 = 200;
        self.setup(base_height);
        /*
        //25 an 100 chosen arbitrarily
        let num_of_waves: u8 = rand::thread_rng().gen_range(25..=100);

        //wave version for temporary variation in terrain
        let base_height: f32 = 200.0;
        let mut x_count: f32 = 0.0;
        let x_period: f32 = rand::thread_rng().gen_range(70.0..=130.0);
        let x_amp: f32 = rand::thread_rng().gen_range(15.0..=45.0);
        let mut x: i32;
        let mut y_count: f32 = 0.0;
        let y_period: f32 = rand::thread_rng().gen_range(70.0..=130.0);
        let y_amp: f32 = rand::thread_rng().gen_range(15.0..=45.0);
        let mut y: i32;
        
        for row in self.map.get_mut_tiles() {
            y = (30.0 * (y_count/100.0).sin()).round() as i32;
            let yx: i32 = (30.0 * ((x_count*y_count)/100.0).sin()).round() as i32;
            for tile in row {
                x = (30.0 * (x_count/100.0).sin()).round() as i32;
                x_count += 1.0;
                if x_count == 2000.0 { x_count = 0.0; }

                let mut xy: f32 = ((0.3 * ((((x_count-1333.0).powi(2) + (y_count-1046.0).powi(2)).sqrt())/100.0).sin()) /*+ (17.0 * ((((x_count-2000.0).powi(2) + (y_count).powi(2)).sqrt())/24.0).sin())*/);

                tile.set_height( ((1.0 + xy) * base_height).round() as i32);
            }
            y_count += 1.0;
            if y_count == 2000.0 { y_count = 0.0; }
        }
        */

        /*
        for row in self.map.get_mut_tiles() {
            let y: i32 = ((46.0 * (y_count/100.0).sin()) + (17.0 * (x_count/234.0).sin())).round() as i32;
            for tile in row {
                let reference_line: i32 = 200;
                let x = ((46.0 * (y_count/100.0).sin()) + (17.0 * (x_count/234.0).sin())).round() as i32;
                tile.set_height(reference_line + x + y);
                x_count += 1.0;
                if x_count == 2000.0 { x_count = 0.0; }
            }
            y_count += 1.0;
            if y_count == 2000.0 { y_count = 0.0; }
        }
        */

        self.generate_map();
        self.extrapolate_image_data();
        self.generate_image();

        println!("Map generated!\n");
        println!("height range: {}", *self.map.get_height_range());
        println!("lowest height: {}", *self.map.get_min_height());
        println!("highest height: {}", *self.map.get_max_height());
        println!("sea level: {}", *self.map.get_sea_level());
        println!("water surface area: {}%", *self.map.get_water_percent_surface_area() * 100.0);

        unsafe {
            Self::mark_gui_dirty("Map generated!".to_string());
        }   
    }

    pub fn setup(&mut self, base_height: i32)  {
        println!("Preparing map...");
        unsafe {
            Self::mark_gui_dirty("Preparing map...".to_string());
        }
        setup::set_up_map(&mut self.map, base_height);
    }

    pub fn generate_map(&mut self) {
        println!("Generating map...");
        unsafe {
            Self::mark_gui_dirty("Generating map...".to_string());
        }   
        MapGenerator::generate_map(&mut self.map);
    }

    pub fn extrapolate_image_data(&mut self) {
        println!("Extrapolotating image data...");
        unsafe {
            Self::mark_gui_dirty("Extrapolating image data...".to_string());
        }
        self.map_image_data = ImageData::interpret_map_data(&mut self.map);
    }
    
    pub fn generate_image(&mut self) {
        println!("Generating image...");
        unsafe {
            Self::mark_gui_dirty("Generating image...".to_string());
        }
        self.map_image_path = ImageGenerator::generate_image(&mut self.map_image_data, &mut self.map_name);
    }

    //GUI interaction, organziation subject to change
    pub unsafe fn mark_gui_dirty(new_status: String) {
        // Attempt to get a mutable reference to the existing observer
        if let Some(dirty_observer) = GUI_OBSERVER.get_mut() {
            dirty_observer.gui_dirty = true;
            dirty_observer.status = new_status;
        } else {
            let mut dirty_observer = Observer::new();
            dirty_observer.gui_dirty = true;
            dirty_observer.status = new_status;
            GUI_OBSERVER.set(dirty_observer).expect("Failed to set GUI_OBSERVER");
        }
    }

    //getters and setters
    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_mut_map(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn get_map_name(&self) -> &String {
        &self.map_name
    }

    pub fn set_map(&mut self, a_map: Map) {
        self.map = a_map;
    }

    pub fn get_status(&self) -> &String {
        &self.status
    }

    pub fn get_image_data(&self) -> &Option<ImageData> {
        &self.map_image_data
    }

    pub fn get_image_path(&self) -> &String {
        &self.map_image_path
    }
}

