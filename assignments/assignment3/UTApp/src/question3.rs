use super::*;
use rand::{thread_rng, Rng};

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    assert_eq!(calculator::get_square_root(x), x.sqrt());
}
#[test]
pub fn test_random_negitive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    assert!(calculator::get_square_root(-x).is_nan());
}
#[test]
pub fn test_square_root_of_zero() {
    assert_eq!(calculator::get_square_root(0.0), 0.0);
}

#[test]
pub fn test_square_root_of_one() {
    assert_eq!(calculator::get_square_root(1.0), 1.0);
}
