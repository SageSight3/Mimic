#[cfg(test)]
use super::*;
use crate::modules::feature_generators::utility::Distribution;
use crate::modules::feature_generators::impact_generator::ImpactGenerator;
use crate::modules::feature_generators::impact_generator::Crater;
use crate::modules::map::Coordinate;
use crate::modules::map::Map;

#[test]
fn test_default() {
    let impact_generator: ImpactGenerator = Default::default();

    assert_eq!(*impact_generator.get_undistributed_height(), 0);
}

#[test]
fn test_crater_new() {
    let trans_rad: u16 = 6;
    let rim_rad: u16 = 11;
    let depth: u16 = 4;
    let width: usize = 100;
    let length: usize = 100;
    let x: usize = 5;
    let y: usize = 7;
    let coords: Vec<Vec<Coordinate>> = vec![vec![Coordinate::new(x, y); width]; length];

    let crater: Crater = Crater::new(trans_rad, rim_rad, depth, coords);

    assert_eq!(*crater.get_transient_radius(), trans_rad);
    assert_eq!(*crater.get_rim_radius(), rim_rad);
    assert_eq!(*crater.get_crater_depth(), depth);

    assert_eq!(crater.get_tile_coords().len(), length);
    for row in crater.get_tile_coords() {
        assert_eq!(row.len(), width);
        for coord in row {
            assert_eq!(*coord.get_X(), x);
            assert_eq!(*coord.get_Y(), y);
        }
    }
    assert_eq!(*crater.get_ejecta_volume(), 0);
}

#[test]
fn test_crater_tiles_coords() {
    //happy path
    let mut a_map: Map = Default::default();
    let mut x: usize = 500;
    let mut y: usize = 400;
    let mut rad: u16 = 20;
    let mut an_impact_coord: Coordinate = Coordinate::new(x, y);

    let mut crater_tiles_coords: Vec<Vec<Coordinate>> = 
        ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    let mut found_edge_tile: bool = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;
            let x_coord: i32 = *coord.get_X() as i32 - x as i32;
            let y_coord: i32 = *coord.get_Y() as i32 - y as i32;
            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);

            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

    /****************************************************************
     * if exceeding bounds, wrap crater around to other side of map
     ***************************************************************/

    //y_coord drops below 0
    x = 500;
    y = 4;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32 - x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32;
            if y_coord > (*a_map.get_length() as i32 / 2) {
                y_coord -= *a_map.get_length() as i32;
            }
            y_coord -= y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

    //x_coord drops below 0
    x = 5;
    y = 400;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);
    
    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
        
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32;
            if x_coord > (*a_map.get_width() as i32 / 2) {
                x_coord -= *a_map.get_width() as i32;
            }
            x_coord -= x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32 - y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

    //x_coord and y_coord drop below 0
    x = 5;
    y = 4;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32;
            if x_coord > (*a_map.get_width() as i32 / 2) {
                x_coord -= *a_map.get_width() as i32;
            }
            x_coord -= x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32;
            if y_coord > (*a_map.get_length() as i32 / 2) {
                y_coord -= *a_map.get_length() as i32;
            }
            y_coord -= y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);
    
    //x_coord exceeds a_map.width()
    x = 1995;
    y = 400;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);
    
    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    let mut iter = 0;
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    println!("outer len: {}", crater_tiles_coords.len());
    for row in crater_tiles_coords {
        iter += 1;
        assert!(row.len() > 0);
        println!("inner len: {}", row.len());
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32;
            if x_coord <= (*a_map.get_width() as i32 / 2) {
                x_coord += *a_map.get_width() as i32;
                println!("x_coord x exceeds map width: {}", x_coord.clone());
            }
            x_coord -= x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32 - y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

    //y_coord exceeds map.length()
    x = 500;
    y = 1994;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32 - x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32;
            if y_coord <= (*a_map.get_length() as i32 / 2) {
                y_coord += *a_map.get_length() as i32;
            }
            y_coord -= y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

    //both x_coord and y_coord exceeds map bounds
    x = 1995;
    y = 1994;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    found_edge_tile = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32;
            if x_coord <= (*a_map.get_width() as i32 / 2) {
                x_coord += *a_map.get_width() as i32;
                println!("x_coord x exceeds map width: {}", x_coord.clone());
            }
            x_coord -= x as i32;

            let mut y_coord: i32 = *coord.get_Y() as i32;
            if y_coord <= (*a_map.get_length() as i32 / 2) {
                y_coord += *a_map.get_length() as i32;
            }
            y_coord -= y as i32;

            assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
            if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                found_edge_tile = true;
            }
        }
    }
    assert!(found_edge_tile);

        //x_coord exceeds map.width(), y_coord drops below 0
        x = 1995;
        y = 5;
        rad = 20;
        an_impact_coord = Coordinate::new(x, y);
    
        crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
        
        found_edge_tile = false;
        assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
        for row in crater_tiles_coords {
            assert!(row.len() > 0);
            for coord in row {
                let rad_int: i32 = rad as i32;
                let mut x_coord: i32 = *coord.get_X() as i32;
                if x_coord <= (*a_map.get_width() as i32 / 2) {
                    x_coord += *a_map.get_width() as i32;
                    println!("x_coord x exceeds map width: {}", x_coord.clone());
                }
                x_coord -= x as i32;
    
                let mut y_coord: i32 = *coord.get_Y() as i32;
                if y_coord > (*a_map.get_length() as i32 / 2) {
                    y_coord -= *a_map.get_length() as i32;
                }
                y_coord -= y as i32;
    
                assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
                if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                    found_edge_tile = true;
                }
            }
        }
        assert!(found_edge_tile);

        //x_coord drops below 0, y_coord exceeds map.length()
        x = 5;
        y = 1994;
        rad = 20;
        an_impact_coord = Coordinate::new(x, y);
    
        crater_tiles_coords = ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
        
        found_edge_tile = false;
        assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
        for row in crater_tiles_coords {
            assert!(row.len() > 0);
            for coord in row {
                let rad_int: i32 = rad as i32;
                let mut x_coord: i32 = *coord.get_X() as i32;
                if x_coord > (*a_map.get_width() as i32 / 2) {
                    x_coord -= *a_map.get_width() as i32;
                    println!("x_coord x exceeds map width: {}", x_coord.clone());
                }
                x_coord -= x as i32;
    
                let mut y_coord: i32 = *coord.get_Y() as i32;
                if y_coord <= (*a_map.get_length() as i32 / 2) {
                    y_coord += *a_map.get_length() as i32;
                }
                y_coord -= y as i32;
    
                assert!(((x_coord.pow(2) as f32 + y_coord.pow(2) as f32).floor()).sqrt() as i32 <= rad_int);
                if (x_coord.pow(2) + y_coord.pow(2) == rad_int.pow(2)) {
                    found_edge_tile = true;
                }
            }
        }
        assert!(found_edge_tile);
}

#[test]
fn test_dig_transient_crater() {
    assert!(false);
}

#[test]
fn test_build_crater_rim() {
    assert!(false);
}

#[test]
fn test_generate_crater() {
    assert!(false);
}

#[test]
fn test_place_undistributed_material() {
    assert!(false);
}

#[test]
fn test_generate() {
    assert!(false);
}