#[cfg(test)]
use super::*;

#[test]
fn test_default() {
    let a_tile = tile::Tile {
        ..Default::default()
    };
    //println!("{}", a_tile.height); //debug
    assert_eq!(a_tile.height, 1);
    
    a_tile.increment_height();

    assert_eq!(a_tile.height, 2);
}