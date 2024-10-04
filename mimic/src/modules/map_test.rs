#[cfg(test)]
use super::*;
use crate::modules::map_attrs;

#[test]
fn test_map_attrs() {
    let a_tile_height = 6;
    let a_map_attrs = map_attrs::MapAttrs {
        base_tile_height: a_tile_height,
        ..Default::default()
    };

    assert_eq!(a_map_attrs.base_tile_height, a_tile_height);
    assert_eq!(a_map_attrs.length, map_attrs::default_length());
    assert_eq!(a_map_attrs.width, map_attrs::default_width());
}

fn make_test_map_attrs() -> map_attrs::MapAttrs {
    let a_map_attrs = map_attrs::MapAttrs {
        ..Default::default()
    };

    a_map_attrs
}

#[test]
fn test_constructor() {
    let a_map_attrs = make_test_map_attrs();
    let a_map = map::Map::new(&a_map_attrs);

    let mut tile_count = 0;
    assert_eq!(a_map.tiles.len(), a_map_attrs.length);
    for row in a_map.tiles{
        assert_eq!(row.len(), a_map_attrs.width);
        for col in row {
            tile_count += 1;
            assert_eq!(col.height, a_map_attrs.base_tile_height);
        }
    }

    let map_length: i32 = a_map_attrs.length.clone() as i32;
    let map_width: i32 = a_map_attrs.width.clone() as i32;
    assert_eq!(tile_count, map_length * map_width);
}

#[test]
fn test_get_tiles() {
    let a_map_attrs = make_test_map_attrs();
    let a_map = map::Map::new(&a_map_attrs);
    assert_eq!(a_map.tiles, *a_map.get_tiles());
}

#[test]
fn test_set_tiles() {
    let a_map_attrs = make_test_map_attrs();
    let mut a_map = map::Map::new(&a_map_attrs);

    let new_map = map::Map::new(
        &map_attrs::MapAttrs {
            base_tile_height: 46,
            length: 100,
            width: 99
        });

    a_map.set_tiles(new_map.tiles.clone());

    assert_eq!(a_map.get_tiles(), new_map.get_tiles());
}

#[test]
fn test_generate_map() {
    let a_map_attrs = make_test_map_attrs();
    let mut a_map = map::Map::new(&a_map_attrs);

    a_map.generate_map();
    for row in a_map.tiles{
        for col in row {
            //based on temporary implementation of map generation
            assert!(*col.get_height() < 256);
        }
    }
}

#[test]
fn set_tile() {
    let a_map_attrs = make_test_map_attrs();
    let mut a_map = map::Map::new(&a_map_attrs);

    let row: usize = 564;
    let col: usize = 764;
    
    assert_eq!(a_map.tiles[row][col].height, a_map_attrs.base_tile_height);

    let new_tile = tile::Tile::new(&46);
    a_map.set_tile(new_tile.clone(), row, col);

    assert_eq!(a_map.tiles[row][col].height, *new_tile.get_height());
}

#[test]
fn get_tile() {
    let a_map_attrs = make_test_map_attrs();
    let mut a_map = map::Map::new(&a_map_attrs);
    let new_tile = tile::Tile::new(&796);
    let row: usize = 865;
    let col: usize = 432;
    a_map.set_tile(new_tile.clone(), row, col);

    assert_eq!(*a_map.get_tile(row, col).get_height(), *new_tile.get_height());
}
