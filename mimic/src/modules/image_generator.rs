use crate::modules::image_interpreter::ImageData;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use crate::modules::image_interpreter::Pixel;
use image::ImageError;

#[derive(Debug)]
pub struct ImageGenerator {
    image_data: ImageData,
    image_path: String,
}

impl ImageGenerator {
    pub fn new(data: &mut Option<ImageData>, name: &mut String) -> ImageGenerator {
        ImageGenerator {
            image_data: data.clone().unwrap(),
            image_path: "..\\maps\\".to_owned() + name + ".png"
        }
    }

    pub fn generate_image(image_data: &mut Option<ImageData>, name: &mut String) -> String {
        let mut image_generator = ImageGenerator::new(image_data, name);
        let image = image_generator.make_image();
        
        let image_path = image_generator.make_file(&image.expect("failed to write image to file"));
        image_path.unwrap()
    }

    pub fn make_image(&self) -> Result<RgbImage, ImageError> {
        let mut image: RgbImage = ImageBuffer::new(*self.image_data.get_width(), *self.image_data.get_length());

        for (x, row) in self.image_data.get_pixels().iter().enumerate() {
            for (y, pixel) in row.iter().enumerate() {
                image.put_pixel(x as u32, y as u32, Rgb([*pixel.get_r(), *pixel.get_g(), *pixel.get_b()]));
            }
        }

        Ok(image)
    }

    pub fn make_file(&mut self, image: &RgbImage) -> Result<String, ImageError> {

        image.save_with_format(self.image_path.clone(), image::ImageFormat::Png);
        Ok(self.image_path.clone())
    }

    pub fn get_image_data(&self) -> &ImageData {
        &self.image_data
    }

    pub fn get_image_path(&self) -> &String {
        &self.image_path
    }
}