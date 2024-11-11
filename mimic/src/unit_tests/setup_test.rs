#[cfg(test)]
use super::*;
use crate::modules::setup;
use crate::modules::setup::SineWave;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use crate::modules::map::Coordinate;

#[test]
fn test_make_sine_wave() {
    let amplitude: f32 = 4.0;
    let period: f32 = 1.0 / 11.0;
    let x_translate: f32 = 444.0;
    let y_translate: f32 = 878.0;

    let sine_wave: SineWave = setup::make_sine_wave(amplitude, period, x_translate, y_translate);

    let x: usize = 56;
    let y: usize = 87;
    let coord: Coordinate = Coordinate::new(x, y);

    let expected_result: f32 = amplitude * (
        (
            (
                ((x as f32 + x_translate).powi(2)) +
                ((y as f32 + y_translate).powi(2))
            ).sqrt()
        ) * period
    ).sin();

    let test_sine_wave: f32 = sine_wave(&coord);

    assert_eq!(test_sine_wave, expected_result);
}

#[test]
fn test_combine() {
    let a_amplitude: f32 = 4.0;
    let a_period: f32 = 1.0 / 11.0;
    let a_x_translate: f32 = 444.0;
    let a_y_translate: f32 = 878.0;

    let a: SineWave = setup::make_sine_wave(a_amplitude, a_period, a_x_translate, a_y_translate);

    let b_amplitude: f32 = 7.0;
    let b_period = 1.0/56.0;
    let b_x_translate = 534.0;
    let b_y_translate = 432.0;

    let b: SineWave = setup::make_sine_wave(b_amplitude, b_period, b_x_translate, b_y_translate);

    let combined: SineWave = setup::combine(a, b);

    let x: usize = 56;
    let y: usize = 87;
    let coord: Coordinate = Coordinate::new(x, y);

    let expected_result: f32 = (a_amplitude * (
        (
            (
                ((x as f32 + a_x_translate).powi(2)) +
                ((y as f32 + a_y_translate).powi(2))
            ).sqrt()
        ) * a_period
    ).sin()) +
    (b_amplitude * (
        (
            (
                ((x as f32 + b_x_translate).powi(2)) +
                ((y as f32 + b_y_translate).powi(2))
            ).sqrt()
        ) * b_period
    ).sin());

    let test_combined: f32 = combined(&coord);
    assert_eq!(test_combined, expected_result);
}

#[test]
fn make_noise_func() {
    let mut test_val: f32 = f32::MAX;

    let x: usize = 56;
    let y: usize = 87;
    let coord: Coordinate = Coordinate::new(x, y);

    let noise_func = setup::make_noise_func(100);

    test_val = noise_func(&coord);

    assert!(test_val != f32::MAX);
}

#[test]
fn test_noisify_height() {
    let base_height: i32 = 200;
    let mut a_map: Map = Default::default();

    setup::noisify_height(&mut a_map, base_height);

    a_map.update_tiles(|tile: &mut Tile| {
        assert!(*tile.get_height() != i32::MAX);
    });
}

#[test]
fn test_set_up_map() {
    let base_height: i32 = 200;
    let mut a_map: Map = Default::default();

    setup::set_up_map(&mut a_map, base_height);

    a_map.update_tiles(|tile: &mut Tile| {
        assert!(*tile.get_height() != i32::MAX);
    });
}