pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt()
}

#[cfg(test)]
mod test {

    macro_rules! assert_eq_approx {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    use super::*;

    #[test]
    fn euclidean_distance() {
        assert_eq_approx!(
            compute_euclidean_distance(&Point::new(2, 4), &Point::new(-3, 8),),
            6.403,
            0.0001
        );
    }
}
