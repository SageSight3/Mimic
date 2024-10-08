//will hide console for finished project, keep commented out for development
//#![cfg_attr(not(debug_assertions), windows_subsystems = "windows")]

/*
Description
*/

use eframe::egui; //GUI crate
use std::sync::OnceLock;
use std::io;
use crate::map_processor;

//https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html#method.set
pub static mut GUI_OBSERVER: OnceLock<Observer> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Observer {
    pub gui_dirty: bool,
    pub status: String,
}

impl Observer {
    pub fn new() -> Observer {
        let temp = "".to_string();
        let observer = Observer {
            gui_dirty: false,
            status: "".to_string()
        };
        observer
    }
}

unsafe fn mark_gui_clean() {
        // Attempt to get a mutable reference to the existing observer
        if let Some(observer) = GUI_OBSERVER.get_mut() {
            observer.gui_dirty = false;
        } else {
            let mut dirty_observer = Observer::new();
            dirty_observer.gui_dirty = true;
            GUI_OBSERVER.set(dirty_observer).expect("Failed to set GUI_OBSERVER");
        }
}

pub fn start_app() -> eframe::Result {
    unsafe { //initialize observer, this is considered unsafe since this is applying a mutable value to a static variable
        GUI_OBSERVER.set(Observer::new());
    }

    env_logger::init(); //will log errors to stderr if RUST_LOG = debug

    //defines behavior of the GUI's window
    let options = eframe::NativeOptions {
        /*egui viewports are native windows. 
        ViewportBuilder is a struct that will define the visual options and data of the window' options
        default will set all nonspecified options to their default*/
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]), //params require an  egui::Vec2 as From<&(f32, f32)>
        ..Default::default() //this is struct update syntax, and sets the remaining options
                             //to their default values specified in impl Default for NativeOptions  
    };

    //start app
    eframe::run_native( //https://docs.rs/eframe/latest/eframe/fn.run_native.html for more info
        "Mimic",
        options,
        //https://github.com/emilk/egui/blob/master/examples/hello_world/src/main.rs
        Box::new(|creation_context| {//|<name>| is closure syntax

            Ok(Box::<MimicGUI>::default())
        })
    )
}

#[derive(Debug, Clone)]
struct MimicGUI {
    display_status: String
}

impl Default for MimicGUI { //default values for MimicGUI
    fn default() -> Self {
        Self {
            display_status: "".to_string(),
            //MapGenerator/MapGenerationObserver
            //pass
            //eon
        }
    }
}

impl eframe::App for MimicGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {  //calls whenever UI needs to be repainted, is required
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Mimic!");
            if ui.button("Generate Map").clicked() {
                /* being kept for future reference in the immediate
                unsafe {
                    map_processor::MapProcessor::mark_gui_dirty("howdy".to_string());
                }
                */
                self.set_display_status("Generating...".to_string());
                //generateMap();
                self.set_display_status("MapGenerated".to_string());
            }
            unsafe {
                let gui_observer_ref = GUI_OBSERVER.get().expect("GUI_OBSERVER get failed in app update()");
                if gui_observer_ref.gui_dirty {
                    self.display_status = gui_observer_ref.status.clone();
                    mark_gui_clean();
                }
            }
            ui.label(format!("{}", self.display_status));
        });
    }


}

impl MimicGUI {
    pub fn set_display_status(&mut self, new_status:String) {
        self.display_status = new_status;
    }
}