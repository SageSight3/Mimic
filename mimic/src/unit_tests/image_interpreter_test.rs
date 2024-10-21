#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;
use crate::modules::image_interpreter::ImageData;
use crate::modules::image_interpreter::Pixel;

#[test]
fn test_pixel_new() {
    let r = 6;
    let g = 54;
    let b = 63;
    let a_pixel = Pixel::new(r, g, b);

    assert_eq!(*a_pixel.get_r(), r);
    assert_eq!(*a_pixel.get_g(), g);
    assert_eq!(*a_pixel.get_b(), b);
}

#[test]
fn test_new() {
    let mut a_map: Map = Default::default();
    let map_image_data = ImageData::new(&mut a_map);

    assert_eq!(map_image_data.get_pixels().len(), *a_map.get_attrs().get_length());

    for row in map_image_data.get_pixels() {
        assert_eq!(row.len(), *a_map.get_attrs().get_width());
        for pixel in row {
            //based on temporary implementation of map generation
            assert_eq!(*pixel.get_r(), 0);
        }
    }
}

#[test]
fn test_interpret_map_data() {
    let mut a_map: Map = Default::default();
    MapGenerator::placeholder_generator(&mut a_map);

    let mut map_image_data: Option<ImageData> = ImageData::interpret_map_data(&mut a_map);

    if let Some(ref image_data) = map_image_data {
        let mut found_high_pixel_value: bool = false;

        for row in map_image_data.expect("map image data failed to unwrap in test_interpret_map_data").get_pixels() {
            for pixel in row {
                assert!(*pixel.get_b() <= 255);
                if *pixel.get_g() > 1 as u8 {
                    found_high_pixel_value = true;
                }
            }
        }
        assert!(found_high_pixel_value);
    } else {
        //if map_image_data was none
        let _fail_test = false;
        assert!(_fail_test);
    }
}

#[test]
fn test_interpret_height_map() {
    let mut a_map: Map = Default::default();
    MapGenerator::placeholder_generator(&mut a_map);

    let mut map_image_data = ImageData::new(&mut a_map);
    map_image_data.interpret_height_map();

    let mut found_high_pixel_value: bool = false;

    for row in map_image_data.get_pixels() {
        for pixel in row {
            //based on temporary implementation of map generation
            assert!(*pixel.get_r() <= 255);
            if *pixel.get_g() > 1 as u8 {
                found_high_pixel_value = true;
            }
        }
    }
    assert!(found_high_pixel_value);
}

#[test]
fn test_interpret_water_map() {
    assert!(false);
}