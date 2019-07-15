use std::f64;
use litcontainers::*;

pub fn sinc(x: f64) -> f64 {
	if x == 0. {
		1.
	} else {
		let pix = f64::consts::PI * x;
		pix.sin() / pix
	}
}

/// Modified first kind bessel function order zero.
/// Credit: Sigpack
/// See bessel functions on [Wikipedia](https://en.wikipedia.org/wiki/Bessel_function)
pub fn besseli0(x: f64) -> f64 {
	let x2 = x * x;
	let mut y = 1.0;
	let mut s = 1.0;
	let mut n = 1;
	while x > y * 1.0e-9 {
		s *= x2 / 4.0 / (n * n) as f64;
		y += s;
		n += 1;
	}
	y
}

