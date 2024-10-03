#[cfg(test)]
use super::*;

#[test]
fn test_constructor() {
    let test_height = 33;
    let a_tile = tile::Tile::new(&test_height);
    //println!("{}", a_tile.height); //debug
    assert_eq!(a_tile.height, test_height);
}

#[test]
fn test_increment() {
    let test_height = 4;
    let mut a_tile = tile::Tile::new(&test_height);
    a_tile.increment_height();
    assert_eq!(*a_tile.get_height(), test_height + 1);
}