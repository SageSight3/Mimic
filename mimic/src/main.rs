mod modules;
mod unit_tests;
use modules::gui as gui;

fn main() {
    gui::start_app().expect("App failed to start");
}