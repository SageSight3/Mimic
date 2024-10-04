//Clone trait provides copying functionality for a type
//Debug provides output in debugging context
//PartialEq implements == and != for a type

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub height: i32
}

impl Tile {

    //constructor
    pub fn new(a_height: &i32) -> Tile {
        Tile {
            height: a_height.to_owned()
        }
    }

    pub fn increment_height(&mut self) {
        self.height += 1;
    }

    pub fn decrement_height(&mut self) {
        self.height -= 1;
    }

    pub fn set_height(&mut self, a_height: i32) {
        self.height = a_height;
    }

    pub fn get_height(&self) -> &i32 {
        &self.height
    }
}