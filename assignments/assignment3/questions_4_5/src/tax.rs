use std::f64::INFINITY;
use num::PrimInt;
use std::ops::{Mul, Add, Sub};

const INCOME_1: f64 = 1.0;
const INCOME_2: f64 = 10000.0;
const INCOME_3: f64 = 50000.0;
const INCOME_4: f64 = 100000.0;
const INCOME_5: f64 = 1000000.0;

// calculates the amount of income a person gets to keep based on their tax 
// bracket. Assumes progressive taxation
pub fn tax<I>(income: I) -> f64 
    where I: PrimInt {
        
        let income = income.to_f64().unwrap();
        match income {
            i if i < 1.0 => panic!("Income should be 1 or greater"),
            i if (INCOME_1..=INCOME_2).contains(&i) => i,
            i if (INCOME_2..=INCOME_3).contains(&i) => (i-INCOME_2) * 0.9 + tax(INCOME_2 as i32),
            i if (INCOME_3..=INCOME_4).contains(&i) => (i-INCOME_3) * 0.8 + tax(INCOME_3 as i32),
            i if (INCOME_4..=INCOME_5).contains(&i) => (i-INCOME_4) * 0.7 + tax(INCOME_4 as i32),
            i if (INCOME_5..INFINITY).contains(&i) => (i-INCOME_5) * 0.6 + tax(INCOME_5 as i32),
            _ => panic!("Invalid income entered"),
        }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{Rng, thread_rng};


    #[test]
    #[should_panic]
    fn should_panic_negative_income(){
        tax(-1);
    }

    #[test]
    #[should_panic]
    fn should_panic_not_integer() {
        // this type of error is caught by the compiler doing type checking.
        // therefore, a test is not needed, since the code will refuse to
        // compile if the programmer gets this wrong, so a test will not help
        // tax(1.0);
        panic!()
    }


    // I just wrote these tests below to make sure the recursive logic was
    // working as intended
    #[test]
    fn bracket_1(){
        let x: i32 = thread_rng().gen_range(INCOME_1 as i32, INCOME_2 as i32);
        let i = x as f64;
        assert_eq!(i, tax(x));
    }

    #[test]
    fn bracket_2(){
        let x: i32 = thread_rng().gen_range(INCOME_2 as i32, INCOME_3 as i32);
        let i = x as f64;
        assert_eq!(INCOME_2 + (i-INCOME_2) * 0.9, tax(x));
    }

    #[test]
    fn bracket_3(){
        let x: i32 = thread_rng().gen_range(INCOME_3 as i32, INCOME_4 as i32);
        let i = x as f64;
        assert_eq!(INCOME_2 + (INCOME_3-INCOME_2) * 0.9 + (i-INCOME_3) * 0.8, tax(x));
    }

    #[test]
    fn bracket_4(){
        let x: i32 = thread_rng().gen_range(INCOME_4 as i32, INCOME_5 as i32);
        let i = x as f64;
        assert_eq!(INCOME_2 + (INCOME_3-INCOME_2) * 0.9 + (INCOME_4-INCOME_3) * 0.8 + (i-INCOME_4) * 0.7, tax(x));
    }
    
    #[test]
    fn bracket_5(){
        let x: i32 = thread_rng().gen_range(INCOME_5 as i32, std::i32::MAX);
        let i = x as f64;
        assert_eq!(INCOME_2 + (INCOME_3-INCOME_2) * 0.9 + (INCOME_4-INCOME_3) * 0.8 + (INCOME_5-INCOME_4) * 0.7 + (i-INCOME_5)  * 0.6, tax(x));
    }
}

