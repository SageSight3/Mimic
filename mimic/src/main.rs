mod modules;
mod unit_tests;
use modules::gui;
//map_mprocessor module is still needed, attributing this to unique strcuture of project compared to how rust projects are usually set up
//ran into first issue without it in gui, look into
use modules::map_processor;
use modules::map_processor::MapProcessor;

fn main() {
    gui::start_app().expect("App failed to start");
    let a_processor: MapProcessor = Default::default();
}