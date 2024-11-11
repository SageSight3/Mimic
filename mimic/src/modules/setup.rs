use crate::modules::map::Coordinate;
use crate::modules::map::Map;
use crate::modules::tile::Tile;
use rand::Rng;

pub type SineWave = Box<dyn Fn(&Coordinate) -> f32>; //since wave closure

pub fn set_up_map(a_map: &mut Map, base_height: i32) {
    noisify_height(a_map, base_height);
}

pub fn noisify_height(a_map: &mut Map, base_height: i32) {
    let noise_func = make_noise_func(30);

    let height_set = |coord: &Coordinate, tile: &mut Tile| {
        let noise: f32 = noise_func(coord).round();
        tile.set_height(base_height + noise as i32);
    };

    a_map.update_tiles_positionally(height_set);
    a_map.compute_height_data();

    //if min height drops below 0, increase map height, so min height is 0
    //do this to prevent overflow errors in genreators only expecting positive heights
    let min_height: i32 = *a_map.get_min_height();
    if min_height < 0 {
        a_map.update_tiles(|tile: &mut Tile| {
            tile.add_height(min_height.abs());
        });
        a_map.compute_height_data();
    }
}

//returns a noise function built from a number of sine waves, called complexity, each wave being built based map attributes
pub fn make_noise_func(complexity: u16) -> impl Fn(&Coordinate) -> f32 {

    //set first wave to 0, to pass as base  arg to combine
    let mut current_noise_func = make_sine_wave(0.0, 1.0, 0.0, 0.0);
    
    for wave in 0..complexity {

        //division slower than multiplication, so will divide before putting into a new SineWave closure
        let period: f32 = 1.0 / rand::thread_rng().gen_range(15.0..=130.0);
        let amp: f32 = rand::thread_rng().gen_range(3.0..=11.0);

        //range chosen arbitrarilt, for now
        //change to map length/width to map length/width/2
        let x_translate: f32 = rand::thread_rng().gen_range(-1999.0..=1000.0);
        let y_translate: f32 = rand::thread_rng().gen_range(-1999.0..=1000.0);

        let new_wave = make_sine_wave(amp, period, x_translate, y_translate);
        current_noise_func = combine(current_noise_func, new_wave);
    }

    current_noise_func
}

pub fn combine(a: Box<dyn Fn(&Coordinate) -> f32>, b: Box<dyn Fn(&Coordinate) -> f32>) -> SineWave {
    Box::new(
        move | a_coord: &Coordinate | {
            a(a_coord) + b(a_coord)
        }
    )
}

pub fn make_sine_wave(amp: f32, period: f32, x_translate: f32, y_translate: f32) -> SineWave {

    //set everythng to be based off a random amp instead, no translation, no period
        //to go from 1d to 2d, have two different frequencies: instead of using the
        //x and y of a coordinate as the x in sin x, they wlll be used to deermine the 
        //frequencies of sinx amp*sin(x*period*frequency) frequecy = f(x and y)?

    //note waves may produce exclusively create circular appearing terrain for now

    Box::new(
        move |a_coord: &Coordinate| {
            amp * (
                (
                    (
                        ((*a_coord.get_X() as f32 + x_translate).powi(2)) +
                        ((*a_coord.get_Y() as f32 + y_translate).powi(2))
                    ).sqrt()
                ) * period
            ).sin()
        } 
    )
 }
