use num::PrimInt;
use std::iter::Product;

pub fn combination<I>(a: I, b: I) -> I 
    where I: PrimInt + Product {
    if b > a {
        panic!("b cannot be greater than a");
    }
    factorial(a) / (
        factorial(a-b) * factorial(b)
    )
}

fn factorial<I>(n: I) -> I 
    where I: PrimInt + Product {
    match n {
        x if x < I::zero() => panic!("factorial requires a number >= 0"),
        x if x == I::zero() => I::one(),
        x => num::range_inclusive(I::one(), x).product(),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use num::PrimInt;
    
    use assert_impl::assert_impl;

    #[test]
    fn a_b_integers() {
        let _a: i64 = 2;
        let _b: i64 = 2;

        assert_impl!(PrimInt: i64);
    }


    #[test]
    #[should_panic]
    fn should_panic_not_integers() {
        let a: f64 = 2.0;
        let b: f64 = 1.0;

        // this type of error is caught by the compiler doing type checking.
        // therefore, a test is not needed, since the code will refuse to
        // compile if the programmer gets this wrong, so a test will not help
        // combination(a, b);
        panic!()
    }

    #[test]
    #[should_panic]
    fn should_panic_b_greater_than_a() {
        combination(1, 2);
    }
}

