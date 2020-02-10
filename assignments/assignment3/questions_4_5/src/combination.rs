use num::PrimInt;
use std::iter::Product;

// this function accepts non-integers but will panic if an integer is given
// this is so that a test can be written which tests that this function panics
// if a non integer is given
// this is kind of dumb since rustc will catch errors like this at compile time
// and there is no need to detect it at runtime
pub fn combination_wrapper(a: f64, b: f64) -> u128 {
    assert_eq!(a.fract(), 0.0);
    assert_eq!(a.fract(), 0.0);
    combination(a as u128, b as u128)
}

pub fn combination<I>(a: I, b: I) -> I
where
    I: PrimInt + Product,
{
    if b > a {
        panic!("b cannot be greater than a");
    }
    factorial(a) / (factorial(a - b) * factorial(b))
}

fn factorial<I>(n: I) -> I
where
    I: PrimInt + Product,
{
    match n {
        x if x < I::zero() => panic!("factorial requires a number >= 0"),
        x if x == I::zero() => I::one(),
        x => num::range_inclusive(I::one(), x).product(),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use assert_impl::assert_impl;
    use num::PrimInt;

    #[test]
    fn a_b_integers() {
        let a: i64 = 2;
        let b: i64 = 2;

        assert_impl!(PrimInt: i64);
        combination(a, b);
        combination_wrapper(a as f64, b as f64);
    }

    #[test]
    #[should_panic]
    fn should_panic_not_integers() {
        let a: f64 = 2.2;
        let b: f64 = 1.5;

        // this type of error is caught by the compiler doing type checking.
        // therefore, a test is not needed, since the code will refuse to
        // compile

        //however, since the assignment demands the application panics at
        // runtime,
        combination_wrapper(a, b);
    }

    #[test]
    #[should_panic]
    fn should_panic_b_greater_than_a() {
        combination(1, 2);
        combination_wrapper(1.0, 2.0);
    }
}
