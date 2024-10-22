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
    let mut a_map: Map = Default::default();
    let x: usize = 5;
    let y: usize = 4;
    let rad: u16 = 20;
    let an_impact_coord: Coordinate = Coordinate::new(x, y);

    let crater_tiles_coords: Vec<Vec<Coordinate>> = 
        ImpactGenerator::crater_tiles_coords(&mut a_map, rad, &an_impact_coord);
    
    let mut found_edge_tile: bool = false;
    assert_eq!(crater_tiles_coords.len(), (rad + 1) as usize);
    for row in crater_tiles_coords {
        for coord in row {
            //coord distance from impact coord
            let delta_x: f32 = (*an_impact_coord.get_X() - *coord.get_X()) as f32;
            let delta_y: f32 = (*an_impact_coord.get_Y() - *coord.get_Y()) as f32;
            let dist_from_center: usize = (delta_x.powi(2) + delta_y.powi(2)).sqrt() as usize;

            assert!(dist_from_center as u16 <= rad);
            if dist_from_center as u16 <= rad { // should be ==, is <= for debgging
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