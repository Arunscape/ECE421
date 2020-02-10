use super::*;
use rand::{thread_rng, Rng};
use roots::{find_roots_quadratic, Roots};

#[test]
pub fn test_basic_roots() {
    assert_eq!(
        calculator::get_roots(1.0, 3.0, -4.0),
        Roots::Two([-4.0, 1.0])
    );
}
#[test]
pub fn test_single_root() {
    assert_eq!(calculator::get_roots(1.0, -4.0, 4.0), Roots::One([2.0]));
}
#[test]
pub fn test_random_solvable_quadratic() {
    loop {
        let (a, b, c) = get_three_random_numbers();

        if !determinant_valid(a, b, c) {
            continue;
        }

        assert_eq!(
            calculator::get_roots(a, b, c),
            find_roots_quadratic(a, b, c)
        );
        return;
    }
}
#[test]
pub fn test_random_non_solvable_quadratic() {
    loop {
        let (a, b, c) = get_three_random_numbers();

        if determinant_valid(a, b, c) {
            continue;
        }

        assert_eq!(calculator::get_roots(a, b, c), Roots::No([]));
        return;
    }
}

#[cfg(test)]
fn get_three_random_numbers() -> (f64, f64, f64) {
    let mut rng = thread_rng();
    (rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}
#[cfg(test)]
fn determinant_valid(a: f64, b: f64, c: f64) -> bool {
    (b.powf(2.0) - 4.0 * a * c) < 0.0
}
