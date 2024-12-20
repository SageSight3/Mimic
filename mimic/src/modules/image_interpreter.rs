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
        let wid: usize = a_map.get_attrs().get_width().to_owned();
        let len: usize = a_map.get_attrs().get_length().to_owned();

        let pixel_vector: Vec<Vec<Pixel>> = vec![vec![Default::default(); wid]; len];

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
        image_data.interpret_water_map();
        Some(image_data)
    }

    pub fn interpret_height_map(&mut self) {

        let max_color_val: i32 = 200; //pixel at coords of max height should be 200, min should b 0
        for rowIndex in 0..self.map.get_tiles().len() {
            let row = &self.map.get_tiles()[rowIndex];
            for colIndex in 0..row.len() {

                //we don't have to worry about negative tile heights, because if the case, min_height will also be negative, meaning
                //the loweest trimmed_height could ever be is 0
                let mut trimmed_height: i32 = (*self.map.get_tile(rowIndex, colIndex).get_height()) - self.map.get_min_height();
                let height_color_value: u8 = (max_color_val as f32 * (trimmed_height as f32/(*self.map.get_height_range() as f32))) as u8;

                self.pixels[rowIndex][colIndex].r = height_color_value;
                self.pixels[rowIndex][colIndex].g = height_color_value;
                self.pixels[rowIndex][colIndex].b = height_color_value;
            }
        }
    }

    pub fn interpret_water_map(&mut self) {

        let trimmed_sea_level: i32 = *self.map.get_sea_level() - self.map.get_min_height();

        for rowIndex in 0..self.map.get_tiles().len() {
            let row = &self.map.get_tiles()[rowIndex];
            for colIndex in 0..row.len() {
                if *self.map.get_tile(rowIndex, colIndex).has_water() {
                    self.pixels[rowIndex][colIndex].r = 0;
                    self.pixels[rowIndex][colIndex].g = 0;

                    let mut trimmed_height: i32 = (*self.map.get_tile(rowIndex, colIndex).get_height()) - self.map.get_min_height();
                    let trimmed_depth: i32 = trimmed_sea_level - trimmed_height;

                    //since water depth range will always be from 0 to trimmed_sea_level, can just use trimmed_sea_level to represent it
                    //tiles at sea level will have a blue value of 255, while the tiles with the gratest depths (tiles at min_height) will
                    //have a blue value of 128
                    let depth_color_val: u8 = (255.0 - (128.0 * (trimmed_depth as f32/trimmed_sea_level as f32))) as u8;

                    self.pixels[rowIndex][colIndex].b = depth_color_val;
                }
            }
        }
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
    b: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue,
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