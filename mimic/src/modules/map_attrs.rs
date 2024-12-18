#[derive(Debug, Clone)]
pub struct MapAttrs {
    length: usize,
    width: usize,
    base_tile_height: i32
}

impl Default for MapAttrs {
    fn default() -> Self {
        Self {
            length: default_length(),
            width: default_width(),
            base_tile_height: default_tile_height()
        }
    }
}

impl MapAttrs {
    pub fn new (len:usize, wid:usize, base_height:i32) -> Self{
        Self {
            length: len,
            width: wid,
            base_tile_height: base_height
        }
    }

    pub fn get_length(&self) -> &usize{
        &self.length
    }

    pub fn get_width(&self) -> &usize{
        &self.width
    }

    pub fn get_base_tile_height(&self) -> &i32 {
        &self.base_tile_height
    }
}

pub fn default_width() -> usize {
    2000
}

pub fn default_length() -> usize {
    2000
}

//the 32-bit integer limit will be used as the default height of a tile
//to signify if any still need a real height generated for them
pub fn default_tile_height() -> i32 {  
    i32::MAX
}