#[cfg(test)]
use super::*;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map_generator::MapGenerator;
use crate::modules::map_processor::MapProcessor;
use crate::modules::image_interpreter::ImageData;
use crate::modules::image_interpreter::Pixel;

#[test]
fn test_new() {
    let a_map: Map = Default::default();
    let a_map_processor = MapProcessor::new(a_map.clone());

    let processor_map_tiles = a_map_processor.get_map().get_tiles();
    assert_eq!(a_map_processor.get_map().get_tiles().len(), a_map.get_tiles().len());
}

#[test]
fn test_generate_map() {
    let mut a_map_processor: MapProcessor = Default::default();

    a_map_processor.generate_map();
    
    let mut generated = false;
    for row in a_map_processor.get_map().get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            if *col.get_height() != 200 {
                generated = true;
            }
        }
    }
    assert!(generated);
}

#[test]
fn test_extrapolate_image_data() {
    let mut a_map_processor: MapProcessor = Default::default();

    //test that a_map_processor.map_image_data is None, if there is something, it fails
    if let Some(ref image_data) = a_map_processor.get_image_data() {
        assert!(false);
    } else {
        assert!(true);
    }

    a_map_processor.extrapolate_image_data();

    if let Some(ref image_data) = a_map_processor.get_image_data() {
        let mut found_high_pixel_value: bool = false;

        for row in image_data.get_pixels() {
            for pixel in row {
                assert!(*pixel.get_b() <= 255);
                if *pixel.get_r() > 1 as u8 {
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
fn test_generate_image() {
    let mut a_map_processor: MapProcessor = Default::default();

    a_map_processor.extrapolate_image_data();
    a_map_processor.generate_image();
    assert!(*a_map_processor.get_image_path() != "".to_string());
}

#[test]
fn test_process_map() {
    let mut a_map_processor: MapProcessor = Default::default();

    //test that a_map_processor.map_image_data is None, if there is something, it fails
    if let Some(ref image_data) = a_map_processor.get_image_data() {
        assert!(false);
    } else {
        assert!(true);
    }

    a_map_processor.process_map();

    //check that map generated
    let mut generated = false;
    for row in a_map_processor.get_map().get_tiles() {
        for col in row {
            //based on temporary implementation of map generation
            if *col.get_height() != 200 {
                generated = true;
            }
        }
    }
    assert!(generated);

    //check that image data was extrapolated
    if let Some(ref image_data) = a_map_processor.get_image_data() {
        
        let mut found_high_pixel_value: bool = false;

        for row in image_data.get_pixels() {
            for pixel in row {
                assert!(*pixel.get_r() <= 255);
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

    let map_img_data_len= a_map_processor.get_image_data().clone().unwrap().get_pixels().len();
    assert_eq!(map_img_data_len, *a_map_processor.get_mut_map().get_attrs().get_length());
    
    //check that map image file was made successfully
    assert!(*a_map_processor.get_image_path() != "".to_string());
}

#[test]
fn test_set_map() {
    let mut a_map_processor: MapProcessor = Default::default();
    let default_map: Map = Default::default();
    let a_map: Map = Map::new(&mut MapAttrs::new(100, 400, 3));

    assert_eq!(a_map_processor.get_map().get_tiles().len(), default_map.get_tiles().len());

    a_map_processor.set_map(a_map.clone());

    assert_eq!(a_map_processor.get_map().get_tiles().len(), a_map.get_tiles().len());
}