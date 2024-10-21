//Clone trait provides copying functionality for a type
//Debug provides output in debugging context
//PartialEq implements == and != for a type

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub height: i32,
    water_present: bool
}

impl Tile {

    //constructor
    pub fn new(a_height: &i32) -> Tile {
        Tile {
            height: a_height.to_owned(),
            water_present: false
        }
    }

    pub fn increment_height(&mut self) {
        self.height += 1;
    }

    pub fn decrement_height(&mut self) {
        self.height -= 1;
    }

    pub fn add_height(&mut self, addend: i32) {
        self.height += addend;
    }

    pub fn remove_height(&mut self, subtrahend: i32) {
        self.height -= subtrahend;
    }

    pub fn set_height(&mut self, a_height: i32) {
        self.height = a_height;
    }

    pub fn get_height(&self) -> &i32 {
        &self.height
    }

    pub fn has_water(&self) -> &bool {
        &self.water_present
    }

    pub fn add_water(&mut self) { //maybe rename
        self.water_present = true;
    }

    pub fn remove_water(&mut self) { //maybe rename
        self.water_present = false;
    }
    
    //look into \/
    //will not be making a getter for a mutable reference to height, as there's not function to mutate height that'd be called by it
    //getters for mutable references should be implemented only if their return type is a data type with non-obvious ways of mutating
    //ex. structs
}