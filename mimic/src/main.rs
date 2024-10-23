//will fix warnings later
#![allow(warnings)]

mod modules;
mod unit_tests;
use modules::app;
//map_mprocessor module is still needed, attributing this to unique strcuture of project compared to how rust projects are usually set up
//ran into first issue without it in gui, look into
use modules::map_processor;
use modules::map_processor::MapProcessor;
use eframe::egui;

fn main() {

    app::start_app().expect("App failed to start");

    /*GUI Multithread resolution idea ->look into mutexes with Rust
    have static/shared mutable var, processing -> maybe reference old GUI Observer to set up
    thread GUI
    if processing: thread map processor
    */    
}