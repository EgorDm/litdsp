use litcontainers::{Dim, RowVec};
use crate::functions;
use std::cmp::max;

#[allow(non_snake_case)]
fn resample<D: Dim>(s: RowVec<f64, D>, p: i32, q: i32) {
	assert!(q > 0 && p > 0, "Factors must be positive");
	let gcd = functions::gcd(p, q);
	let (p, q) = (p / gcd, q / gcd);

	if q == p { return; }
	let K = max(p, q);
}