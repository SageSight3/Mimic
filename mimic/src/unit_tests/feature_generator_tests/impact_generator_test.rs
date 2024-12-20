use std::cmp::max;

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
    for row in crater_tiles_coords {
        iter += 1;
        assert!(row.len() > 0);
        for coord in row {
            let rad_int: i32 = rad as i32;

            let mut x_coord: i32 = *coord.get_X() as i32;
            if x_coord <= (*a_map.get_width() as i32 / 2) {
                x_coord += *a_map.get_width() as i32;
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
    //running 100 trials to ensure confidence across a large number of possible crater variances
    for trial in 0..50 {
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
        let an_impact_coord_height: u16 = *a_map.get_tile(y, x).get_height() as u16;
        let crater_depth: u16 = rand::thread_rng().gen_range(1..=40);
        let vari: f32 = rand::thread_rng().gen_range(3.0..=4.0);
        let trans_rad: f32 = (vari*(crater_depth as f32))/3.0;
        let rim_rad: f32 = (trans_rad* (1.3 + rand::thread_rng().gen_range(0.0..=0.7))).round();
        //convert rads to u16
        let trans_rad: u16 = trans_rad.round() as u16;
        let rim_rad: u16 = rim_rad.round() as u16;

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
            let height_diff: f32 =  ((ax_divisor/ax_dividend) - c).round();
            let height: u16 = ((an_impact_coord_height as f32 + height_diff).floor()) as u16;
            
            trans_crater_height_range.push(height);
        }

        //test
        assert!(trans_crater_height_range.len() > 0);
        assert_eq!(trans_crater_height_range[0], an_impact_coord_height - (crater_depth));
        a_crater.dig_transient_crater(&mut a_map);
        for (mut distance, coords_at_dist) in a_crater.get_tiles_coords().iter().enumerate() {
            if distance == trans_crater_height_range.len() { break; }

            for coord in coords_at_dist {
                let tile_to_test: &Tile = a_map.get_tile(*coord.get_Y(), *coord.get_X());

                if *tile_to_test.get_height() <= trans_crater_height_range[distance] as i32 {
                }
                assert!(*tile_to_test.get_height() <= trans_crater_height_range[distance] as i32);
            }
        }
        assert!(*a_crater.get_ejecta_volume() > 0);
    }
}

#[test]
fn test_build_crater_rim() {
    //running 100 trials to ensure confidence across a large number of possible crater variances
    for trial in 0..50 {
        //setup
        let mut a_map: Map = Default::default();
        a_map.update_tiles(|a_tile: &mut Tile| {
            a_tile.set_height(200);
        });

        //impact coordinate
        let x: usize = 500;
        let y: usize = 400;
        let mut an_impact_coord: Coordinate = Coordinate::new(x, y);
        
        //the formation of the upward slope from the transient crater should also form ejecta, but if the
        //heights of tiles in the upwards slope is less than the tile's at the impact coordinate, no 
        //ejecta would be added. So to test that ejecta addition is working, set the impact coord's 
        //manually to ensure there is material to throw out.
        a_map.get_mut_tile(y, x).set_height(100);

        //Make crater
        //depth, radii, and variance
        let an_impact_coord_height: u16 = *a_map.get_tile(y, x).get_height() as u16;
        let crater_depth: u16 = rand::thread_rng().gen_range(1..=40);
        let vari: f32 = rand::thread_rng().gen_range(3.0..=4.0);
        let trans_rad: f32 = (vari*(crater_depth as f32))/3.0;
        let rim_rad: f32 = (trans_rad * (1.3 + rand::thread_rng().gen_range(0.0..=0.7))).round();

        //convert rads to u16
        let trans_rad: u16 = trans_rad.round() as u16;
        let rim_rad: u16 = rim_rad.round() as u16;

        let tiles_coords: Vec<Vec<Coordinate>> = 
            ImpactGenerator::crater_tiles_coords(&mut a_map, trans_rad + rim_rad, &an_impact_coord);

        let mut a_crater: Crater = Crater::new(vari, trans_rad, rim_rad,an_impact_coord_height, crater_depth, tiles_coords);

        /************************************
        * get predicted added_height range *
        ************************************/

        let mut rim_added_height_range: Vec<u16> = Vec::new();
        let max_added_height: f32 = ((0.1) * (2.0 * trans_rad as f32));
        let rounded_max: f32 = max_added_height.round();

        //upward slope to max_added_height from crater center
        let ax_dividend: f32 = (crater_depth as f32) * (vari.powi(2));
        let c: f32 = crater_depth as f32;
        let mut max_height_dist: u16 = trans_rad;

        for dist_from_center in (trans_rad + 1)..=rim_rad {
            let ax_divisor: f32 = 9.0 * (dist_from_center as f32).powi(2);
            let height_to_add: f32 =  ((ax_divisor/ax_dividend) - c).round();
            if height_to_add > rounded_max {
                max_height_dist = dist_from_center - 1;
                break;
            }
            
            //convert height_to_add to a u16
            let height_to_add: u16 = height_to_add as u16;
            rim_added_height_range.push(height_to_add);
        }

        //test upward slope height range from crater center
        for height_to_add in rim_added_height_range.clone() {
            assert!(height_to_add <= rounded_max as u16);
        }

        //downward slope from max_height_dist to crater edge, refer to rim_height_math.png for name definitions
        let md: u16 = max_height_dist - trans_rad;
        let dividend: f32 = (trans_rad as f32 + md as f32);

        for dist_from_center in (max_height_dist + 1)..=rim_rad {
            let divisor: f32 = max_added_height * (dist_from_center as f32 - (trans_rad as f32 - md as f32));

            let height_to_add: u16 = (max_added_height as f32 - (divisor/dividend)).round() as u16;
            rim_added_height_range.push(height_to_add);
        }

        //test downward slope height range from crater center
        for height_to_add in rim_added_height_range.clone() {
            //println!("{}", height_to_add);
            assert!(rounded_max as u16 >= height_to_add && height_to_add >= 0);
        }

        /************************
         * test build_crater_rim
         ************************/
        
        //get coordinates to be tested
        let mut rim_tiles: Vec<Vec<Coordinate>> = Vec::new();

        for index in (trans_rad as usize + 1)..=rim_rad as usize {
            rim_tiles.push(a_crater.get_tiles_coords()[index].clone());

            //make sure rim_tiles added the row successfully
            assert_eq!(rim_tiles[index - (trans_rad as usize + 1)].len(), a_crater.get_tiles_coords()[index].len());
        }

        //make sure rim_tiles added the all target rows, + 1 to rim_rad - (trans_rad + 1) due to len being one bigger than range of
        //rim_tiles' slice of a_crater.tiles_coords
        assert_eq!(rim_tiles.len(), (1 + rim_rad-(trans_rad + 1)) as usize);

        let mut old_tile_heights: Vec<Vec<i32>> = Vec::new();
        for (index,row) in rim_tiles.iter().enumerate() {
            old_tile_heights.push(Vec::new());
            for coord in row {
                let an_old_height: i32 = a_map.get_tile(*coord.get_Y(), *coord.get_X()).get_height().clone();
                old_tile_heights[index].push(an_old_height);
                //println!("{}", an_old_height);
            }
            assert_eq!(old_tile_heights[index].len(), row.len());
        }
        assert_eq!(old_tile_heights.len(), rim_tiles.len());

        a_crater.build_crater_rim(&mut a_map);
        
        //test height at each tile in crater rim
        for (rowIndex, row) in rim_tiles.iter().enumerate() {
            let distance: u16 = trans_rad + 1 + rowIndex as u16;
            for (coordIndex, coord) in row.iter().enumerate() {
                let tile_to_test: &Tile = a_map.get_tile(*coord.get_Y(), *coord.get_X());
                if distance <= max_height_dist {
                    assert_eq!(*tile_to_test.get_height(), (an_impact_coord_height as i32 + rim_added_height_range[rowIndex] as i32));  
                } else {
                    assert_eq!(*tile_to_test.get_height(), (old_tile_heights[rowIndex][coordIndex] + rim_added_height_range[rowIndex] as i32));
                }
            }
        }

        //sometimes max_height_dist will equal trans_rad, in which case, no ejecta will be thrown in
        //the formation of the crater rim, Note: due to extreme difference in impact_coord_height
        //and the height of the rest of the tiles in the map, ejecta volume, when generated, will likely
        //be far larger than it would typically be outside of the test case
        if trans_rad != max_height_dist {
            assert!(*a_crater.get_ejecta_volume() > 0);
        } else {
            assert_eq!(*a_crater.get_ejecta_volume(), 0);
        }
    }
}

#[test]
fn test_generate_crater() {
    let mut a_map: Map = Default::default();
    a_map.update_tiles(|a_tile: &mut Tile| {
        a_tile.set_height(200);
    });

    let mut an_impact_generator: ImpactGenerator = Default::default();

    assert_eq!(*an_impact_generator.get_undistributed_height(), 0);

    //impact coordinate
    let x: usize = 500;
    let y: usize = 400;
    let mut an_impact_coord: Coordinate = Coordinate::new(x, y);
    let crater_depth: u16 = rand::thread_rng().gen_range(1..=40);

    an_impact_generator.generate_crater(&mut a_map, crater_depth, an_impact_coord);

    assert!(*an_impact_generator.get_undistributed_height() > 0);
}

#[test]
fn test_place_undistributed_material() {
    let mut a_map: Map = Default::default();
    a_map.update_tiles(|a_tile: &mut Tile| {
        a_tile.set_height(200);
    });

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 200);
    });

    let mut an_impact_generator: ImpactGenerator = Default::default();

    an_impact_generator.set_undistributed_height(4000000);
    an_impact_generator.place_undistributed_material(&mut a_map);

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 201);
    });
}

#[test]
fn test_generate() {
    let mut a_map: Map = Default::default();
    a_map.update_tiles(|a_tile: &mut Tile| {
        a_tile.set_height(200);
    });

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 200);
    });

    let impacts_num: u16 = 600;
    let depth_distro: Distribution = Distribution::new(1, 50, 33);

    a_map.update_tiles(| a_tile: &mut Tile | {
        assert_eq!(*a_tile.get_height(), 200);
    });

    ImpactGenerator::generate(&mut a_map, impacts_num, &depth_distro);
    
    let mut tile_height_changed: bool = false;
    for row in a_map.get_tiles(){
        for col in row {
            if *col.get_height() != 200 {
                tile_height_changed = true;
            }
        }
    }

    assert!(tile_height_changed);
}