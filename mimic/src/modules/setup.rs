use crate::modules::map::Coordinate;
use rand::Rng;

pub fn make_noise_func() -> impl Fn(&Coordinate) -> f32 {
    
    //25 an 100 chosen arbitrarily
    let num_of_waves: u8 = rand::thread_rng().gen_range(25..=100);

    let mut current_noise_func = make_sine_wave();
    
    for wave in 1..num_of_waves {
        let new_wave = make_sine_wave();
        current_noise_func = combine(current_noise_func, new_wave);
    }

    current_noise_func
}

pub fn combine(a: Box<dyn Fn(&Coordinate) -> f32>, b: Box<dyn Fn(&Coordinate) -> f32>) -> Box<dyn Fn(&Coordinate) -> f32>
{
    Box::new(
        move | a_coord: &Coordinate | {
            a(a_coord) + b(a_coord)
        }
    )
}

pub fn make_sine_wave() -> Box<dyn Fn(&Coordinate) -> f32> {
    let period: f32 = rand::thread_rng().gen_range(70.0..=130.0);
    let amp: f32 = rand::thread_rng().gen_range(15.0..=45.0);

    //set everythng to be based off a random amp instead, no translation, no period
        //to go from 1d to 2d, have two different frequencies: instead of using the
        //x and y of a coordinate as the x in sin x, they wlll be used to deermine the 
        //frequencies of sinx amp*sin(x*period*frequency) frequecy = f(x and y)?

    //range chosen arbitrarilt, for now
    //change to map length/width to map length/width/2
    let x_translate: f32 = rand::thread_rng().gen_range(-1999.0..=1000.0);
    let y_translate: f32 = rand::thread_rng().gen_range(-1999.0..=1000.0);

    //note waves may produce exclusively create circular appearing terrain for now

    Box::new(
        move |a_coord: &Coordinate| {
            amp * (
                (
                    (
                        ((*a_coord.get_X() as f32 + x_translate).powi(2)) +
                        ((*a_coord.get_Y() as f32 + y_translate).powi(2))
                    ).sqrt()
                ) / period
            ).sin()
        } 
    )
 }
