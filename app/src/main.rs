mod complex;

use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::complex::Complex;

fn main() {
    println!("Hello, world!");

    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);

    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}
