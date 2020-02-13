#![allow(dead_code)]
use num::Float;
use roots::{find_roots_quadratic, FloatType, Roots};
use std::ops::{Add, Div, Mul, Sub};
pub fn add<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

pub fn subtract<T>(x: T, y: T) -> T
where
    T: Sub<Output = T>,
{
    x - y
}

pub fn multiply<T>(x: T, y: T) -> T
where
    T: Mul<Output = T>,
{
    x * y
}

pub fn divide<T>(x: T, y: T) -> T
where
    T: Div<Output = T>,
{
    x / y
}

pub fn get_square_root<T>(x: T) -> T
where
    T: Float,
{
    x.sqrt()
}

// ax^2 + bx + c = 0
pub fn get_roots<F>(a: F, b: F, c: F) -> Roots<F>
where
    F: FloatType,
{
    find_roots_quadratic(a, b, c)
}
