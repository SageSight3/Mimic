mod modules;
use modules::gui as gui;

fn main() {
    gui::start_app().expect("App failed to start");
}