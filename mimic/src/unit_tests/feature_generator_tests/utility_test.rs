use super::*;
use crate::modules::feature_generators::utility::Distribution;

#[cfg(test)]
mod distribution_test {
    use crate::modules::feature_generators::utility::Distribution;

    #[test]
    fn test_rand() {
        let a_min: u16 = 46; let a_max: u16 = 89; let mut a_skew: i16 = -7;
        let mut a_distribution: Distribution = Distribution::new(a_min, a_max, a_skew);
        let mut a_num: u16 = 0;

        a_num = a_distribution.rand();
        assert!(a_num >= *a_distribution.get_min());
        assert!(a_num < *a_distribution.get_max());

        assert_eq!(*a_distribution.get_skew(), a_skew);
        assert_eq!(*a_distribution.get_min(), a_min);
        assert_eq!(*a_distribution.get_max(), a_max);

        a_skew = 8;
        a_distribution = Distribution::new(a_min, a_max, a_skew);
        assert_eq!(*a_distribution.get_skew(), a_skew);
    }
}
