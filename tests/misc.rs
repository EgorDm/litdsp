use litcontainers::*;
use litdsp::*;

#[test]
pub fn resample() {
	let freq = 1.;
	let fr = 6.;
	let s = wave::generate_wave(freq, U40, 0, fr, false);

	let sa = resample::resample(s, 2, 1);
}