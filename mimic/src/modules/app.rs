//will hide console for finished project, keep commented out for development
//#![cfg_attr(not(debug_assertions), windows_subsystems = "windows")]

/*
Description
*/

use eframe::egui; //GUI crate
use std::io;
use std::thread;
use std::sync::mpsc;
use std::thread::Scope;
use std::time::Duration;
use crate::map_processor::MapProcessor; //is needed for GUI to signal map processor to begin

pub fn start_app() -> eframe::Result {
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

            Ok(Box::<Mimic>::default())
        })
    )
}

#[derive(Debug)]
struct Mimic { //struct that holds full application
    map_processor: MapProcessor
}

impl Default for Mimic { //default values for MimicGUI
    fn default() -> Self {
        Self {
            map_processor: Default::default(),
        }
    }
}

impl eframe::App for Mimic {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {  //calls whenever UI needs to be repainted, is required
        //MULTITHREADING STILL NOT MULTITHREADING
        //https://www.youtube.com/watch?v=3bFSrhm4nEM
        let display_status = self.map_processor.get_status().clone();

        //is map processing?
        let (sender, reciever) = mpsc::channel();

        //closure for gui thread
        let gui = || {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Welcome to Mimic!");
                if ui.button("Generate Map").clicked() {
                    //println!("{}", thread::current().id().as_u64()); //nightly only
                    //signal to start map processor thread - NOT WORKING
                    sender.send(true).expect("Button press failed to send signal to start map processor thread");
                } else {
                    sender.send(false).expect("Button press failed to send signal to skip starting map processor thread");
                }
                //the label itself acts as an observer
                ui.label(format!("{}", display_status));
            });
        };

        //map processor thread
        let map_processor_thread = || {
            //signal that map processor thread has started
            //println!("{}", thread::current().id().as_u64()); //nightly only
            sender.send(false).expect("Failed to signal the GUI that map processor thread has started");
            self.map_processor.process_map();
        };

        thread::scope(|scope: &Scope<'_, '_>| { 
            scope.spawn(gui);
            if (reciever.recv().expect("Failed to recieve signal to start map processor thread")) {
                scope.spawn(map_processor_thread);
            }
        }); 
    }
}