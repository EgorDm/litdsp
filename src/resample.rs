use litcontainers::*;
use crate::{functions, filters, window};
use std::cmp::max;

#[allow(non_snake_case)]
pub fn resample<D>(s: RowVec<f64, D>, p: i32, q: i32)
	where D: Dim
{
	assert!(q > 0 && p > 0, "Factors must be positive");
	let gcd = functions::gcd(p, q);
	let (p, q) = (p / gcd, q / gcd);

	if q == p { return; }
	let K = max(p, q);

	let n = 10;
	let fac = 0.5 / K as f64;
	let f = rvec!(U4, &[0., 2. * fac, 2. * fac, 1.]);
	let a = rvec!(U4, &[1., 1., 0., 0.]);
	let l = (2 * n * K + 1) as usize;
	let mut filter = filters::firls(D!(l - 1), f, a);
	let window = window::kaiser(D!(l), 5.);
	filter *= &(window * p as f64);

	let length = s.col_count();
	let length_half = (length - 1) / 2;
	let output_size = functions::quotient_ceil(length * p as usize, q as usize);



	println!("{}", filter);
}