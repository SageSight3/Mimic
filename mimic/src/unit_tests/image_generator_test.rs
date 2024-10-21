#[cfg(test)]
use super::*;
use crate::modules::image_generator::ImageGenerator;
use crate::modules::image_interpreter::ImageData;
use crate::modules::map::Map;

#[test]
fn test_new() {
    let mut a_map: Map = Default::default();
    let mut image_data: Option<ImageData> = Some(ImageData::new(&mut a_map));
    let mut image_gen: ImageGenerator = ImageGenerator::new(&mut image_data, &mut "new_test".to_string());

    assert_eq!(image_gen.get_image_data().get_length(), image_data.unwrap().get_length());
    assert_eq!(*image_gen.get_image_path(), "..\\maps\\new_test.png");
}

#[test]
fn test_make_image() {
    let mut a_map: Map = Default::default();
    let mut image_data: Option<ImageData> = Some(ImageData::new(&mut a_map));
    let mut image_gen: ImageGenerator = ImageGenerator::new(&mut image_data, &mut "make_image_test".to_string());

    let image = image_gen.make_image();

    match image {
        Ok(RgbImage) => assert!(true),
        Err(ImageError) => assert!(false)
    }
}

#[test]
fn test_make_file() {
    let mut a_map: Map = Default::default();
    let mut image_data: Option<ImageData> = Some(ImageData::new(&mut a_map));
    let mut image_gen: ImageGenerator = ImageGenerator::new(&mut image_data, &mut "make_file_test".to_string());

    let image_path = image_gen.make_file(&image_gen.make_image().unwrap());

    match image_path {
        Ok(String) => assert!(true),
        Err(ImageError) => assert!(false)
    }
}

#[test]
fn test_generate_image() {
    let mut a_map: Map = Default::default();
    let mut image_data: Option<ImageData> = Some(ImageData::new(&mut a_map));

    let mut name = "generate_image_test".to_string();
    let image_path = ImageGenerator::generate_image(&mut image_data, &mut name);

    assert_eq!(image_path, "..\\maps\\".to_owned() + &name + ".png");
}