#[cfg(test)]
use super::*;
use crate::modules::tile::Tile;

#[test]
fn test_constructor() {
    let test_height = 33;
    let a_tile = Tile::new(&test_height);
    assert_eq!(a_tile.height, test_height);
    assert_eq!(*a_tile.has_water(), false);
}

#[test]
fn test_increment() {
    let test_height = 4;
    let mut a_tile = Tile::new(&test_height);
    a_tile.increment_height();
    assert_eq!(*a_tile.get_height(), test_height + 1);
}

#[test]
fn test_decrement() {
    let test_height = 3;
    let mut a_tile = Tile::new(&test_height);
    a_tile.decrement_height();
    assert_eq!(*a_tile.get_height(), test_height - 1);
}

#[test]
fn test_set_height() {
    let test_height = 8;
    let mut a_tile = Tile::new(&test_height);
    a_tile.set_height(test_height);
    assert_eq!(*a_tile.get_height(), test_height);
}

#[test]
fn test_get_height() {
    let test_height = 9;
    let a_tile = Tile::new(&test_height);
    assert_eq!(*a_tile.get_height(), test_height);
}

#[test]
fn test_add_water() {
    let test_height = 0;
    let mut a_tile = Tile::new(&test_height);

    a_tile.add_water();
    assert_eq!(*a_tile.has_water(), true);
}

#[test]
fn test_remove_water() {
    let test_height = 0;
    let mut a_tile = Tile::new(&test_height);
    a_tile.add_water();

    a_tile.remove_water();
    assert_eq!(*a_tile.has_water(), false);
}

