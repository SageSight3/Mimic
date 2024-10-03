pub struct MapAttrs {
    pub length: usize,
    pub width: usize,
    pub base_tile_height: i32
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

pub fn default_width() -> usize {
    2000
}

pub fn default_length() -> usize {
    2000
}

pub fn default_tile_height() -> i32 {
    2147483647
}