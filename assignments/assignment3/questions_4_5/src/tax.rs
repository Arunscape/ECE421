use std::convert::Into

pub fn tax<I>(income: I) -> f64 
    where I: Into<f64> {

        const BRACKET_1: f64 = 1.0;
        const BRACKET_2: f64 = 10000.0;
        const BRACKET_3: f64 = 50000.0;
        const BRACKET_4: f64 = 100000.0,
        const BRACKET_5: f64 = 1000000.0;

        let income = income.into();

        match income {
            i if i < 1 => panic!("Income should be 1 or greater"),
            i if BRACKET_5 < i => (i-BRACKET_5) * 0.4 + tax(BRACKET_5),
            (BRACKET_4..=BRACKET_5) => (income-BRACKET_4) * 0.3 + tax(BRACKET_4),
            (BRACKET_3..=BRACKET_4) => (income-BRACKET_3) * 0.2 + tax(BRACKET_2),
            (BRACKET_2..=BRACKET_3) => (income-BRACKET_2) * 0.1 + tax(BRACKET_1),
            (BRACKET_1..=BRACKET_2) => 0,
        }
}

#[cfg(test)]
mod test {

    use super::*;
}

