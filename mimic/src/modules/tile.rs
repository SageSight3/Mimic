#[derive(Copy)]
pub struct Tile {
    pub height: i32
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            height: 0
        }
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        *self
    }
}

impl Tile {
    pub fn increment_height(mut self) {
    }
}