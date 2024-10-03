#[derive(Debug, Clone)]
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

    pub fn get_height(&self) -> &i32 {
        &self.height
    }
}