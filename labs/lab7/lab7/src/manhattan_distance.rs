extern "C" {
    pub fn abs(i: i32) -> i32;
}

#[repr(C)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}
pub fn compute_manhattan_distanceC(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        (a_abs + b_abs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_distance() {
        assert_eq!(
            3,
            compute_manhattan_distanceC(&Point { x: 2, y: 3 }, &Point { x: 3, y: 5 }),
        )
    }
}
