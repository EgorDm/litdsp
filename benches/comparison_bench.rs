#[macro_use]
extern crate criterion;

mod helpers;
mod benchmarks;

use benchmarks::*;

criterion_main!(
	resample::benchmark,
	stft::benchmark,
	fourier_coefficients::benchmark,
	time_scaling::benchmark,
);