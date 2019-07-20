use std::f64;
use num_traits::PrimInt;
use crate::constants::EPSILON;


/// sinc function [wiki](https://en.wikipedia.org/wiki/Sinc_function)
pub fn sinc(x: f64) -> f64 {
	if x.abs() < EPSILON {
		1.
	} else {
		let pix = f64::consts::PI * x;
		pix.sin() / pix
	}
}

/// Modified first kind bessel function order zero.
/// Credit: Sigpack
/// See bessel functions on [wiki](https://en.wikipedia.org/wiki/Bessel_function)
pub fn besseli0(x: f64) -> f64 {
	let x2 = x * x;
	let mut y = 1.0;
	let mut s = 1.0;
	let mut n = 1;
	while s > y * 1.0e-9 {
		s *= x2 / 4.0 / (n * n) as f64;
		y += s;
		n += 1;
	}
	y
}

/// Greatest common divisor [wiki](https://en.wikipedia.org/wiki/Greatest_common_divisor)
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
	let mut tmp;
	while a > 0 {
		tmp = a;
		a = b % a;
		b = tmp;
	}
	b
}

/// Greatest common divisor on generic integers [wiki](https://en.wikipedia.org/wiki/Greatest_common_divisor)
pub fn gcd_t<T: PrimInt + Default>(mut a: T, mut b: T) -> T {
	let mut tmp;
	while a > T::default() {
		tmp = a;
		a = b % a;
		b = tmp;
	}
	b
}

/// Fancy ceil(a / b)
pub fn quotient_ceil(a: usize, b: usize) -> usize {
	if a % b != 0 {
		a / b + 1
	} else {
		a / b
	}
}