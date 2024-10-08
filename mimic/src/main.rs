mod modules;
mod unit_tests;
use modules::gui;
use modules::map_processor;

fn main() {
    gui::start_app().expect("App failed to start");
    let a_processor: map_processor::MapProcessor = Default::default();
}