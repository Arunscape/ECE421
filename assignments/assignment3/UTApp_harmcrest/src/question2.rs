use rand::{thread_rng, Rng};
use super::*;
use hamcrest2::prelude::*;

#[test]
pub fn basic_multiply() {
    assert_that!(calculator::multiply(2.0, 3.0), eq(6.0));
}
#[test]
pub fn multiply_negative_number() {
    assert_that!(calculator::multiply(4.0, -5.0), eq(-20.0));
}
#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    assert_that!(calculator::multiply(x,y), eq(x*y));
}

#[test]
pub fn basic_divide() {
    assert_that!(calculator::divide(7.0, 2.0), eq(3.5));
}
#[test]
pub fn divide_negative_number() {
    assert_that!(calculator::divide(12.0, -3.0), eq(-4.0));
}
#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    assert_that!(calculator::divide(x,y), eq(x/y));
}
   
