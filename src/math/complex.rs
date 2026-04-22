use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    // based on Euler's formula
    pub fn from_angle(angle: f32) -> Self {
        Self {
            re: angle.cos(),
            im: angle.sin(),
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_from_angle() {
        let num = Complex::from_angle(PI);

        assert!((num.re - (-1.0)).abs() < 1e-5);
        assert!((num.im - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_magnitude() {
        let num = Complex::new(3.0, 4.0);
        assert_eq!(num.magnitude(), 5.0);

        let num_neg = Complex::new(-3.0, -4.0);
        assert_eq!(num_neg.magnitude(), 5.0);
    }

    #[test]
    fn test_addition() {
        let num_1 = Complex::new(1.0, 2.0);
        let num_2 = Complex::new(2.0, 1.0);

        let sum = num_1 + num_2;
        assert_eq!(sum, Complex::new(3.0, 3.0));
    }

    #[test]
    fn test_substraction() {
        let num_1 = Complex::new(1.0, 2.0);
        let num_2 = Complex::new(2.0, 1.0);

        let sub = num_1 - num_2;
        assert_eq!(sub, Complex::new(-1.0, 1.0));
    }

    #[test]
    fn test_multiplication() {
        let num_1 = Complex::new(1.0, 2.0);
        let num_2 = Complex::new(3.0, 4.0);

        let mul = num_1 * num_2;
        assert_eq!(mul, Complex::new(-5.0, 10.0));
    }
}
