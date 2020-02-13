use super::*;
use rand::{thread_rng, Rng};

#[test]
pub fn basic_multiply() {
    assert_eq!(calculator::multiply(2.0, 3.0), 6.0);
}
#[test]
pub fn multiply_negative_number() {
    assert_eq!(calculator::multiply(4.0, -5.0), -20.0);
}
#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    assert_eq!(calculator::multiply(x, y), x * y);
}

#[test]
pub fn basic_divide() {
    assert_eq!(calculator::divide(7.0, 2.0), 3.5);
}
#[test]
pub fn divide_negative_number() {
    assert_eq!(calculator::divide(12.0, -3.0), -4.0);
}
#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    assert_eq!(calculator::divide(x, y), x / y);
}
