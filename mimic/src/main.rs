//will hide console for finished project, keep commented out for development
//#![cfg_attr(not(debug_assertions), windows_subsystems = "windows")]

/*
Description
*/

use eframe::egui; //GUI crate

fn main() -> eframe::Result {
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

struct MimicGUI {

}

impl Default for MimicGUI { //default values for MimicGUI
    fn default() -> Self {
        Self {

        }
    }
}

impl eframe::App for MimicGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {  //calls whenever UI needs to be repainted, is required
        let mut map_generated_msg: &str = "";
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Mimic!");
            if ui.button("Generate Map").clicked() {
                //generateMap();
                map_generated_msg = "Map generated!";
            }
            ui.label(format!("{map_generated_msg}"));
        });
    }
}