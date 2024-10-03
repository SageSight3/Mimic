#[cfg(test)]
use super::*;

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

#[test]
fn test_constructor() {
    let a_map_attrs = map_attrs::MapAttrs {
        ..Default::default()
    };
    let a_map = map::Map::new(&a_map_attrs);

    let mut tile_count = 0;
    assert_eq!(a_map.map.len(), a_map_attrs.length);
    for row in a_map.map{
        assert_eq!(row.len(), a_map_attrs.width);
        for col in row {
            tile_count += 1;
            assert_eq!(col.height, a_map_attrs.base_tile_height);
        }
    }

    let map_length: i32 = a_map_attrs.length.clone() as i32;
    let map_width: i32 = a_map_attrs.width.clone() as i32;
    assert_eq!(tile_count, map_length * map_width)
}