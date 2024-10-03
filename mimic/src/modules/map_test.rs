#[cfg(test)]
use super::*;

#[test]
fn test_default() {
    let a_map = map::Map {
        ..Default::default()
    };
    
    let mut sum: i32 = 0;
    for row in a_map.height_map.iter() {
        for tile in row {
            assert_eq!(tile.height, 0);
            tile.increment_height();
            assert_eq!(tile.height, 1);
            sum += tile.height;
        }
    }
    assert_eq!(sum, 414 * 414);
}