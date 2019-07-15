use litcontainers::*;
use litdsp::*;

#[test]
fn generate_wave() {
	let w = wave::generate_wave(1., U20, 0, 20., false);
	dbg!(w.as_slice());
}

#[test]
fn calculate_time() {
	let sr = 1.;
	let t = wave::calculate_time(U7, sr);
	assert_eq!(t.as_slice(), [0., 1., 2., 3., 4., 5., 6.]);
}