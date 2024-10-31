use rand::Rng;

/* Distribution
Distributions will be used to define ranges for various parameters in different generators
A range will be defined by a minimum and maximum value, while the skew will be used as a way
of creating bias towards a specific mean within the defined range for random number generation
*/
#[derive(Debug, Clone)]
pub struct Distribution { //nothing in this program should require more than a u16
    min: u16,
    max: u16,
    skew: i16
}

impl Distribution {
    pub fn new(a_min: u16, a_max: u16, a_skew: i16) -> Distribution{
        Distribution {
            min: a_min,
            max: a_max,
            skew: a_skew
        }
    }

    pub fn rand(&self) -> u16 {
        let mut a_num: u16 = rand::thread_rng().gen_range(self.min..self.max);

        let bias: u16 = rand::thread_rng().gen_range(0..self.skew.abs() as u16);
        if self.skew < 0 as i16 {
            if bias >= a_num { 
                a_num = self.min;
            } else {
                a_num -= bias;
            }
        } else {
            if a_num > (self.max / 2) && bias > (self.max / 2)  {
                //limiting a_num to max - 1 to keep with coding convention of range start being inclusive but end being exclusive
                a_num = self.max - 1;
            } else {
                a_num += bias;
            }
        }
        a_num
    }

    pub fn get_min(&self) -> &u16 {
        &self.min
    }

    pub fn get_max(&self) -> &u16 {
        &self.max
    }

    pub fn get_skew(&self) -> &i16 {
        &self.skew
    }
}