use crate::modules::app::Observer;
use crate::modules::app::GUI_OBSERVER;
use std::sync::OnceLock;
use crate::modules::map::Map;
use crate::modules::map_generator::MapGenerator;
use crate::modules::image_interpreter::ImageData;
use crate::modules::image_generator::ImageGenerator;
use std::io;

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
            map_name: "placeholder".to_string(),
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
        //parse_specification(a_map_specification);
        self.generate_map();
        self.extrapolate_image_data();
        self.generate_image();
        unsafe {
            Self::mark_gui_dirty("Map generated!".to_string());
        }   
    }

    pub fn generate_map(&mut self) {
        unsafe {
            Self::mark_gui_dirty("Generating map...".to_string());
        }   
        MapGenerator::generate_map(&mut self.map);
    }

    pub fn extrapolate_image_data(&mut self) {
        unsafe {
            Self::mark_gui_dirty("Extrapolating image data...".to_string());
        }
        self.map_image_data = ImageData::interpret_map_data(&mut self.map);
    }
    
    pub fn generate_image(&mut self) {
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

