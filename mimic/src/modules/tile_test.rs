#[cfg(test)]
use super::*;

#[test]
fn test_default() {
    let a_tile = tile::Tile {
        ..Default::default()
    };
    //println!("{}", a_tile.height); //debug
    assert_eq!(a_tile.height, 0)
}