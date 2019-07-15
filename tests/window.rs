use litcontainers::*;
use litdsp::*;

#[test]
fn hanning() {
	let w = window::hanning(U7);
	assert_eq!(w.as_slice(), [0.0, 0.24999999999999994, 0.7499999999999999, 1.0, 0.7500000000000002, 0.24999999999999994, 0.0]);
}

#[test]
fn sinc() {
	let w = window::sinw(U7, 1.);
	assert_eq!(w.as_slice(), [0.0, 0.4338837391175581, 0.7818314824680298, 0.9749279121818236, 0.9749279121818236, 0.7818314824680299, 0.43388373911755823]);
	// TODO: actually compare to the output of my c++ lib
}