#![allow(dead_code)]
use num::PrimInt;
use std::f64::INFINITY;

const INCOME_1: f64 = 1.0;
const INCOME_2: f64 = 10000.0;
const INCOME_3: f64 = 50000.0;
const INCOME_4: f64 = 100000.0;
const INCOME_5: f64 = 1000000.0;

// this function accepts non-integers but will panic if an integer is given
// this is so that a test can be written which tests that this function panics
// if a non integer is given
// this is kind of dumb since rustc will catch errors like this at compile time
// and there is no need to detect it at runtime
pub fn tax_wrapper(income: f64) -> f64 {
    assert_eq!(income.fract(), 0.0);
    tax(income as u32)
}

// in case the question wanted the taxed amount, and not the net income after tax
pub fn taxed_amount(income: f64) -> f64 {
    income - tax_wrapper(income)
}

// calculates the amount of income a person gets to keep based on their tax
// bracket. Assumes progressive taxation.
// For example, if your income is 20,000, you get to keep:
// 10,000 + (20,000 - 10,000) * 0.9 = 19,000
// because from 0 to 10,000 is untaxed, while 10,000 to 20,000 is taxed at 10%
pub fn tax<I>(income: I) -> f64
where
    I: PrimInt,
{
    let income = income.to_f64().unwrap();
    match income {
        i if i < 1.0 => panic!("Income should be 1 or greater"),
        i if (INCOME_1..=INCOME_2).contains(&i) => i,
        i if (INCOME_2..=INCOME_3).contains(&i) => (i - INCOME_2) * 0.9 + tax(INCOME_2 as u32),
        i if (INCOME_3..=INCOME_4).contains(&i) => (i - INCOME_3) * 0.8 + tax(INCOME_3 as u32),
        i if (INCOME_4..=INCOME_5).contains(&i) => (i - INCOME_4) * 0.7 + tax(INCOME_4 as u32),
        i if (INCOME_5..INFINITY).contains(&i) => (i - INCOME_5) * 0.6 + tax(INCOME_5 as u32),
        _ => panic!("Invalid income entered"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    #[should_panic]
    fn should_panic_negative_income() {
        tax(-1);
        tax_wrapper(-1.0);
        taxed_amount(-1.0);
    }

    #[test]
    #[should_panic]
    fn should_panic_not_integer() {
        let income: f64 = 2.2;

        // this type of error is caught by the compiler doing type checking.
        // therefore, a test is not needed, since the code will refuse to
        // compile

        //however, since the assignment demands the application panics at
        // runtime,
        tax_wrapper(income);
        taxed_amount(income);
    }

    // I just wrote these tests below to make sure the recursive logic was
    // working as intended, they were not required for the assignment
    #[test]
    fn bracket_1() {
        let x: u32 = thread_rng().gen_range(INCOME_1 as u32, INCOME_2 as u32);
        let i = x as f64;
        assert_eq!(i, tax(x));
    }

    #[test]
    fn bracket_2() {
        let x: u32 = thread_rng().gen_range(INCOME_2 as u32, INCOME_3 as u32);
        let i = x as f64;
        assert_eq!(INCOME_2 + (i - INCOME_2) * 0.9, tax(x));
    }

    #[test]
    fn bracket_3() {
        let x: u32 = thread_rng().gen_range(INCOME_3 as u32, INCOME_4 as u32);
        let i = x as f64;
        assert_eq!(
            INCOME_2 + (INCOME_3 - INCOME_2) * 0.9 + (i - INCOME_3) * 0.8,
            tax(x)
        );
    }

    #[test]
    fn bracket_4() {
        let x: u32 = thread_rng().gen_range(INCOME_4 as u32, INCOME_5 as u32);
        let i = x as f64;
        assert_eq!(
            INCOME_2
                + (INCOME_3 - INCOME_2) * 0.9
                + (INCOME_4 - INCOME_3) * 0.8
                + (i - INCOME_4) * 0.7,
            tax(x)
        );
    }
    #[test]
    fn bracket_5() {
        let x: u32 = thread_rng().gen_range(INCOME_5 as u32, std::u32::MAX);
        let i = x as f64;
        assert_eq!(
            INCOME_2
                + (INCOME_3 - INCOME_2) * 0.9
                + (INCOME_4 - INCOME_3) * 0.8
                + (INCOME_5 - INCOME_4) * 0.7
                + (i - INCOME_5) * 0.6,
            tax(x)
        );
    }
}
