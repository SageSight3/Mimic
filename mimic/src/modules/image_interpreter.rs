use crate::modules::map::Map;

#[derive(Debug, Clone)]
pub struct ImageData {
    pixels: Vec<Vec<u8>>, //eventually will be a double vector of lists of 3 or 4 u8s (rgb or rgba)
    map: Map
}

impl ImageData {

    //Interpretation process subject to change
    pub fn new(a_map: &mut Map) -> ImageData {
        let width = a_map.get_attrs().get_width().to_owned();
        let length = a_map.get_attrs().get_length().to_owned();

        let pixel_vector = vec![vec![0; width]; length];

        ImageData {
            pixels: pixel_vector,
            map: a_map.clone()
        }
    }

    pub fn interpret_map_data(a_map: &mut Map)  -> Option<ImageData> {
        let mut image_data = ImageData::new(a_map);
        image_data.interpret_height_map();
        Some(image_data)
    }

    pub fn interpret_height_map(&mut self) {
        for rowIndex in 0..self.map.get_tiles().len() {
            let row = &self.map.get_tiles()[rowIndex];
            for colIndex in 0..row.len() {
                //we mod 256 as, at least a temporary way, to convert height into a grayscale value
                //subject to change, assumes height range of map is between 0 and 255 inclusively
                self.pixels[rowIndex][colIndex] = (self.map.get_tile(rowIndex, colIndex).get_height() % 256) as u8;
            }
        }
    }

    //getters, there will be no mutable getters or setters, as image data shouldn't change once it's set
    pub fn get_pixels(&self) -> &Vec<Vec<u8>> {
        &self.pixels
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> &u8 {
        &self.pixels[row][col]
    }
}