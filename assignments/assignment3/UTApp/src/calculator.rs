use std::ops::{Add, Sub, Mul, Div};
use num::Float;
    
pub fn add<T>(x: T, y: T) -> T
where T: Add<Output=T> {
    x + y
}

pub fn subtract<T>(x: T, y: T) -> T
where T: Sub<Output=T> {
    x - y
}

pub fn multiply<T>(x: T, y: T) -> T
where T: Mul<Output=T> {
    x * y
}

pub fn divide<T>(x: T, y: T) -> T
where T: Div<Output=T> {
    x / y
}

pub fn get_square_root<T>(x: T) -> T
where T: Float {
    x.sqrt()
}
