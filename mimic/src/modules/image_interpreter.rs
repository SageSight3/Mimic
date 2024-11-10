use crate::modules::map::Map;

#[derive(Debug, Clone)]
pub struct ImageData {
    pixels: Vec<Vec<Pixel>>, //eventually will be a double vector of lists of 3 or 4 u8s (rgb or rgba)
    length: u32,
    width: u32,
    map: Map
}

impl ImageData {

    //Interpretation process subject to change
    pub fn new(a_map: &mut Map) -> ImageData {
        let wid = a_map.get_attrs().get_width().to_owned();
        let len = a_map.get_attrs().get_length().to_owned();

        let pixel_vector = vec![vec![Pixel::new(0, 0, 0); wid]; len];

        ImageData {
            pixels: pixel_vector,
            length: len as u32,
            width: wid as u32,
            map: a_map.clone()
        }
    }

    pub fn interpret_map_data(a_map: &mut Map) -> Option<ImageData> {
        let mut image_data = ImageData::new(a_map);
        image_data.interpret_height_map();
        Some(image_data)
    }

    pub fn interpret_height_map(&mut self) {

        let mut min_height: i32 = 99999999;
        let mut max_height: i32 = -999999999;

        for rowIndex in 0..self.map.get_tiles().len() {
            let row = &self.map.get_tiles()[rowIndex];
            for colIndex in 0..row.len() {
                let a_height: i32 = (*self.map.get_tile(rowIndex, colIndex).get_height()).abs();

                if a_height < min_height { min_height = a_height; }
                if a_height > max_height { max_height = a_height; }
            }
        }

        let mut height_range: i32 = max_height - min_height;
        println!("height range: {}", height_range);

        for rowIndex in 0..self.map.get_tiles().len() {
            let row = &self.map.get_tiles()[rowIndex];
            for colIndex in 0..row.len() {

                //look into desiredHeight = (max_height - min_height)/DesiredRanged * height + min_height 
                let mut trimmed_height: i32 = (*self.map.get_tile(rowIndex, colIndex).get_height()).abs() - min_height;

                let height_color_value: u8 = (200.0 * (trimmed_height as f32/height_range as f32)) as u8;

                self.pixels[rowIndex][colIndex].r = height_color_value;
                self.pixels[rowIndex][colIndex].g = height_color_value;
                self.pixels[rowIndex][colIndex].b = height_color_value;

                /*
                //we mod 256 as, at least a temporary way, to convert height into a grayscale value
                //subject to change, assumes height range of map is between 0 and 255 inclusively
                let height_color_value = (self.map.get_tile(rowIndex, colIndex).get_height() % 256) as u8;
                self.pixels[rowIndex][colIndex].r = height_color_value;
                self.pixels[rowIndex][colIndex].g = height_color_value;
                self.pixels[rowIndex][colIndex].b = height_color_value;
                */
            }
        }
    }

    pub fn interpret_water_map(&mut self) {
        
    }

    //getters, there will be no mutable getters or setters, as image data shouldn't change once it's set
    pub fn get_pixels(&self) -> &Vec<Vec<Pixel>> {
        &self.pixels
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> &Pixel {
        &self.pixels[row][col]
    }

    pub fn get_length(&self) -> &u32 {
        &self.length
    }

    pub fn get_width(&self) -> &u32 {
        &self.width
    }
}

#[derive(Debug, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue
        }
    }

    pub fn get_r(&self) -> &u8 {
        &self.r
    }

    pub fn get_g(&self) -> &u8 {
        &self.g
    }

    pub fn get_b(&self) -> &u8 {
        &self.b
    }
}