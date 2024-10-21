#[cfg(test)]
use super::*;
use crate::modules::map_attrs;
use crate::modules::map_attrs::MapAttrs;
use crate::modules::map::Map;
use crate::modules::map::Coordinate;
use crate::modules::tile::Tile;

#[test]
fn test_map_attrs() {
    let a_tile_height = 6;
    let a_map_attrs = MapAttrs::new(map_attrs::default_length(), map_attrs::default_width(), a_tile_height);

    assert_eq!(*a_map_attrs.get_base_tile_height(), a_tile_height);
    assert_eq!(*a_map_attrs.get_length(), map_attrs::default_length());
    assert_eq!(*a_map_attrs.get_width(), map_attrs::default_width());
}

#[test]
fn test_default() {
    let default_map_attrs: MapAttrs = Default::default();
    let a_map: Map = Default::default();

    assert_eq!(a_map.get_tiles().len(), *default_map_attrs.get_length());
    for row in a_map.get_tiles() {
        assert_eq!(row.len(), *default_map_attrs.get_width());
    }
}

#[test]
fn test_constructor() {
    let mut a_map_attrs: MapAttrs = Default::default();
    let a_map = Map::new(&mut a_map_attrs);

    let mut tile_count = 0;
    assert_eq!(a_map.get_tiles().len(), *a_map_attrs.get_length());
    for row in a_map.get_tiles(){
        assert_eq!(row.len(), *a_map_attrs.get_width());
        for col in row {
            tile_count += 1;
            assert_eq!(col.height, *a_map_attrs.get_base_tile_height());
        }
    }

    let map_length: i32 = a_map_attrs.get_length().clone() as i32;
    let map_width: i32 = a_map_attrs.get_width().clone() as i32;
    assert_eq!(tile_count, map_length * map_width);
}

#[test]
fn test_set_tiles() {
    //let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();

    let new_map = Map::new(&mut MapAttrs::new(100, 99, 46));

    a_map.set_tiles(new_map.get_tiles().clone());

    assert_eq!(a_map.get_tiles(), new_map.get_tiles());
}

#[test]
fn test_set_tile() {
    let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();

    let row: usize = 564;
    let col: usize = 764;
    
    assert_eq!(*a_map.get_tile(row, col).get_height(), *a_map_attrs.get_base_tile_height());

    let new_tile = Tile::new(&46);
    a_map.set_tile(new_tile.clone(), row, col);

    assert_eq!(*a_map.get_tile(row, col).get_height(), *new_tile.get_height());
}

#[test]
fn test_get_tile() {
    //let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();
    let new_tile = Tile::new(&796);
    let row: usize = 865;
    let col: usize = 432;
    a_map.set_tile(new_tile.clone(), row, col);

    assert_eq!(*a_map.get_tile(row, col).get_height(), *new_tile.get_height());
}

#[test]
fn test_get_mut_tiles() {
    //let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();
    let a_height: i32 = 4;
    for row in a_map.get_mut_tiles() {
        for col in row {
            col.set_height(a_height);
            //based on temporary implementation of map generation
            assert_eq!(*col.get_height(), a_height);
        }
    }
}

#[test]
fn test_get_mut_tile() {
    //let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();
    let a_height: i32 = 7;
    let row: usize = 445;
    let col: usize = 412;

    a_map.get_mut_tile(row, col).set_height(a_height);

    assert_eq!(*a_map.get_tile(row, col).get_height(), a_height);
}

#[test]
fn test_new_coordinate() {
    let x: usize = 56;
    let y: usize = 55;

    let a_coordinate: Coordinate = Coordinate::new(x, y);

    assert_eq!(*a_coordinate.get_X(), x);
    assert_eq!(*a_coordinate.get_Y(), y);
}

#[test]
fn test_random_coordinate() {
    let a_map_attrs: MapAttrs = Default::default();
    let mut a_map: Map = Default::default();
    
    let a_coordinate: Coordinate = a_map.random_coordinate();

    assert!(*a_coordinate.get_X() < *a_map.get_attrs().get_width());
    assert!(*a_coordinate.get_Y() < *a_map.get_attrs().get_length());
    assert!(*a_coordinate.get_X() >= 0);
    assert!(*a_coordinate.get_Y() >= 0);
}

#[test]
fn test_update_tiles() {
    let height = 200;
    let mut a_map_attrs: MapAttrs = MapAttrs::new(map_attrs::default_length(), map_attrs::default_width(), height);
    let mut a_map: Map = Map::new(&mut a_map_attrs);

    //control test
    for row in a_map.get_tiles(){
        for col in row {
            assert_eq!(*col.get_height(), height);
        }
    }

    //test with passing update_tiles() a function
    a_map.update_tiles(inc_tiles_height); //inc_tiles_height declared below
    for row in a_map.get_tiles(){
        for col in row {
            assert_eq!(*col.get_height(), height + 1);
        }
    }

    //test with passing update_tiles() a closure
    let reset_height = |a_tile: &mut Tile| { a_tile.set_height(height); };
    a_map.update_tiles(reset_height);

    for row in a_map.get_tiles(){
        for col in row {
            assert_eq!(*col.get_height(), height);
        }
    }
}

fn inc_tiles_height(a_tile: &mut Tile) {
    a_tile.increment_height()
}

#[test]
fn test_water_volume() {
    let mut a_map: Map = Default::default();
    
    assert_eq!(*a_map.get_water_volume(), 0);

    //mutable getter test
    let mut new_volume: u64 = 3;
    *a_map.get_mut_water_volume() = new_volume;
    assert_eq!(*a_map.get_water_volume(), new_volume);

    //setter test
    new_volume = 53;
    a_map.set_water_volume(&new_volume);
    assert_eq!(*a_map.get_water_volume(), new_volume);
}