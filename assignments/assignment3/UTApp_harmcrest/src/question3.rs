use super::*;
use rand::{thread_rng, Rng};
use hamcrest2::prelude::*;

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    assert_that!(calculator::get_square_root(x), eq(x.sqrt()));
}
#[test]
pub fn test_random_negitive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    assert_that!(calculator::get_square_root(-x).is_nan(), is(true));
}
#[test]
pub fn test_square_root_of_zero() {
    assert_that!(calculator::get_square_root(0.0), eq(0.0));
}

#[test]
pub fn test_square_root_of_one() {
    assert_that!(calculator::get_square_root(1.0), eq(1.0));
}
