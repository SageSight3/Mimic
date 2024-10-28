#[cfg(test)]
use super::*;
use crate::modules::feature_generators::utility::Distribution;
use crate::modules::feature_generators::impact_generator::ImpactGenerator;
use crate::modules::feature_generators::impact_generator::Crater;
use crate::modules::map::Coordinate;
use crate::modules::map::Map;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::tile::Tile;
use rand::Rng;

#[test]
fn test_default() {
    let impact_generator: ImpactGenerator = Default::default();

    assert_eq!(*impact_generator.get_undistributed_height(), 0);
}

#[test]
fn test_crater_new() {
    //settup
    let vari: f32 = 3.4434343;
    let trans_rad: u16 = 6;
    let rim_rad: u16 = 11;
    let impact_height: u16 = 3;
    let depth: u16 = 4;
    let width: usize = 100;
    let length: usize = 100;
    let x: usize = 5;
    let y: usize = 7;
    let coords: Vec<Vec<Coordinate>> = vec![vec![Coordinate::new(x, y); width]; length];

    //test
    let crater: Crater = Crater::new(vari, trans_rad, rim_rad, impact_height ,depth, coords);

    assert_eq!(*crater.get_variance(), vari);
    assert_eq!(*crater.get_transient_radius(), trans_rad);
    assert_eq!(*crater.get_rim_radius(), rim_rad);
    assert_eq!(*crater.get_impact_height(), impact_height);
    assert_eq!(*crater.get_crater_depth(), depth);

    assert_eq!(crater.get_tiles_coords().len(), length);
    for row in crater.get_tiles_coords() {
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
    //setup
    let mut a_map: Map = Default::default();
    let mut x: usize = 500;
    let mut y: usize = 400;
    let mut rad: u16 = 20;
    let mut an_impact_coord: Coordinate = Coordinate::new(x, y);

    //test
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
    //setup
    x = 500;
    y = 4;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    //test
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
    //setup
    x = 5;
    y = 400;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);
    
    //test
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
    //setup
    x = 5;
    y = 4;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    //test
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
    
    //setup
    //x_coord exceeds a_map.width()
    x = 1995;
    y = 400;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);
    
    //test
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
                //println!("x_coord x exceeds map width: {}", x_coord.clone()); //debug
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
    //setup
    x = 500;
    y = 1994;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    //test
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
    //setup
    x = 1995;
    y = 1994;
    rad = 20;
    an_impact_coord = Coordinate::new(x, y);

    //test
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
        //setup
        x = 1995;
        y = 5;
        rad = 20;
        an_impact_coord = Coordinate::new(x, y);
    
        //test
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
        //setup
        x = 5;
        y = 1994;
        rad = 20;
        an_impact_coord = Coordinate::new(x, y);
    
        //test
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
    //setup
    let mut a_map: Map = Default::default();
    a_map.update_tiles(|a_tile: &mut Tile| {
        a_tile.set_height(200);
    });

    //impact coordinate
    let x: usize = 500;
    let y: usize = 400;
    let mut an_impact_coord: Coordinate = Coordinate::new(x, y);

    //Make crater
    //depth, radii, and variance
    let an_impact_coord_height: u16 = *a_map.get_tile(x, y).get_height() as u16;
    let crater_depth: u16 = 15;
    let vari: f32 = rand::thread_rng().gen_range(3.0..=4.0);
    let trans_rad: f32 = (vari*(crater_depth as f32))/3.0;
    let rim_rad: f32 = (trans_rad* (1.3 + rand::thread_rng().gen_range(0.0..=0.7))).round();
    //convert rads to u16
    let trans_rad: u16 = trans_rad as u16;
    let rim_rad: u16 = rim_rad as u16;
    //println!("trans_rad as u16: {}", trans_rad); debug
    //println!("rim_rad as u16: {}", rim_rad); //debug

    let tiles_coords: Vec<Vec<Coordinate>> = 
        ImpactGenerator::crater_tiles_coords(&mut a_map, trans_rad + rim_rad, &an_impact_coord);

    let mut a_crater: Crater = Crater::new(vari, trans_rad, rim_rad,an_impact_coord_height, crater_depth, tiles_coords);

    //get predicted height range
    let mut trans_crater_height_range: Vec<u16> = Vec::new();
    //var names from ax^2 - c
    let ax_dividend: f32 = (crater_depth as f32) * (vari.powi(2));
    let c: f32 = crater_depth as f32;
    for dist_from_center in 0..=trans_rad {
        let ax_divisor: f32 = 9.0 * (dist_from_center as f32).powi(2);
        
        //find height at dist_from_center from impact_coordinate
        let height_diff: f32 =  (ax_divisor/ax_dividend) - c;
        //println!("height_diff at dist, {}: {}", dist_from_center, height_diff);
        let height: u16 = ((an_impact_coord_height as f32 + height_diff).floor()) as u16;
        //println!("height at dist, {}: {}", dist_from_center, height);
        
        trans_crater_height_range.push(height);
        //println!("height in vector at, {}: {}", dist_from_center, trans_crater_height_range[dist_from_center as usize]);
    }

    //test
    println!("impact height: {}", an_impact_coord_height);
    assert!(trans_crater_height_range.len() > 0);
    assert_eq!(trans_crater_height_range[0], an_impact_coord_height - (crater_depth));
    a_crater.dig_transient_crater(&mut a_map);
    for (mut distance, coords_at_dist) in a_crater.get_tiles_coords().iter().enumerate() {
        //println!("distance: {}", distance);
        if distance == trans_crater_height_range.len() { break; }

        for coord in coords_at_dist {
            let tile_to_test: &Tile = a_map.get_tile(*coord.get_X(), *coord.get_Y());

            if *tile_to_test.get_height() <= trans_crater_height_range[distance] as i32 {
                //println!("tile_height: {}", tile_to_test.get_height());
            }
            assert!(*tile_to_test.get_height() <= trans_crater_height_range[distance] as i32);
        }
    }
    assert!(*a_crater.get_ejecta_volume() > 0);
}

#[test]
fn test_build_crater_rim() {

    //setup
    let mut a_map: Map = Default::default();
    a_map.update_tiles(|a_tile: &mut Tile| {
        a_tile.set_height(200);
    });

    //impact coordinate
    let x: usize = 500;
    let y: usize = 400;
    let mut an_impact_coord: Coordinate = Coordinate::new(x, y);

    //Make crater
    //depth, radii, and variance
    let an_impact_coord_height: u16 = *a_map.get_tile(x, y).get_height() as u16;
    let crater_depth: u16 = 15;
    let vari: f32 = rand::thread_rng().gen_range(3.0..=4.0);
    let trans_rad: f32 = (vari*(crater_depth as f32))/3.0;
    let rim_rad: f32 = (trans_rad* (1.3 + rand::thread_rng().gen_range(0.0..=0.7))).round();

    //convert rads to u16
    let trans_rad: u16 = trans_rad as u16;
    let rim_rad: u16 = rim_rad as u16;

    let tiles_coords: Vec<Vec<Coordinate>> = 
        ImpactGenerator::crater_tiles_coords(&mut a_map, trans_rad + rim_rad, &an_impact_coord);

    let mut a_crater: Crater = Crater::new(vari, trans_rad, rim_rad,an_impact_coord_height, crater_depth, tiles_coords);

    //get predicted added_height range
    let mut rim_added_height_range: Vec<u16> = Vec::new();
    for dist_from_center in (trans_rad + 1)..=rim_rad {

    }

    //test
    //...
    
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