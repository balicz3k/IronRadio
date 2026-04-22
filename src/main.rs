mod math;

use crate::math::complex::Complex;

fn main() {
    let a = Complex::new(1.2, 4.4);
    let b = Complex::from_angle(6.4);
    let c = a.magnitude();

    println!("{:?}, {:?}, {}", a, b, c);
}
