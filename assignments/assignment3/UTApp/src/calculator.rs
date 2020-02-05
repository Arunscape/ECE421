use std::ops::{Add, Sub};


    
pub fn add<T>(x: T, y: T) -> T
where T: Add<Output=T> + Copy + Clone {
    x + y
}

pub fn subtract<T>(x: T, y: T) -> T
where T: Sub<Output=T> + Copy + Clone {
    x - y
}
