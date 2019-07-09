use litcontainers::*;
use litdsp::*;

#[test]
fn hanning() {
	let w = window::hanning_f64(U7);
	assert_eq!(w.as_slice(), [0.0, 0.24999999999999994, 0.7499999999999999, 1.0, 0.7500000000000002, 0.24999999999999994, 0.0])
}