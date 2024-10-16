use crate::modules::app::Observer;
use crate::modules::app::GUI_OBSERVER;
use std::sync::OnceLock;
use crate::modules::map::Map;
use crate::modules::map_generator::MapGenerator;
use std::io;

#[derive(Debug, Clone)]
pub struct MapProcessor {
    map: Map,
    status: String
    //map_specification: MapSpecification,
    //generation_tasks: Vec<GenerationTask>,
    //map_image_data: ImageData, //data to be passed to map/image interpreter, may not be necessary
    //generated_map_image: Image, //look into how would be set up
    //state: //figure out
    //StateDescription: String //may not be used
}

impl Default for MapProcessor {
   fn default() -> Self {
        Self {
            map: Default::default(),
            status: "Map generator ready!".to_string()
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
        //parse_specification(a_map_spedification);
        self.generate_map();
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

    pub fn set_map(&mut self, a_map: Map) {
        self.map = a_map;
    }

    pub fn get_status(&self) -> &String {
        &self.status
    }
}

