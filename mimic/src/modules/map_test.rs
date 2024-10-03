#[cfg(test)]
use super::*;

#[test]
fn test_default() {
    let a_map = map::Map {
        ..Default::default()
    };

    for row in a_map.height_map.iter() {
        for tile in row {
            assert_eq!(tile.height, 0)
        }
    }
}