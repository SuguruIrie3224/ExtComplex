use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;


// 複素数の構造体
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    re: f64,
    im: f64,
}

// 複素数のメソッド
impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn conj(&self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

// 複素数の四則演算
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re : self.re * rhs.re - self.im * rhs.im,
            im : self.re * rhs.im + self.im * rhs.re,
        }
    }

}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 4.0 };
        let result = a + b;
        assert_eq!(result.re, 4.0);
        assert_eq!(result.im, 6.0);
    }

    #[test]
    fn test_subtraction() {
        let a = Complex { re: 5.0, im: 6.0 };
        let b = Complex { re: 2.0, im: 3.0 };
        let result = a - b;
        assert_eq!(result.re, 3.0);
        assert_eq!(result.im, 3.0);
    }

    #[test]
    fn test_multiplication() {
        let a = Complex { re: 2.0, im: 3.0 };
        let b = Complex { re: 4.0, im: 5.0 };
        let result = a * b;
        assert_eq!(result.re, -7.0);
        assert_eq!(result.im, 22.0);
    }

    #[test]
    fn test_negation() {
        let a = Complex { re: 1.0, im: 2.0 };
        let result = -a;
        assert_eq!(result.re, -1.0);
        assert_eq!(result.im, -2.0);
    }

    #[test]
    fn test_conjugate() {
        let a = Complex { re: 2.0, im: 3.0 };
        let result = a.conj();
        assert_eq!(result.re, 2.0);
        assert_eq!(result.im, -3.0);
    }
}