use litcontainers::*;
use litdsp::*;

#[test]
pub fn resample() {
	let freq = 1.;
	let fr = 6.;
	let s = wave::generate_wave(freq, U10, 0, fr, false);
	let sa = resampling::resample(&s, 2, 1);

	assert_eq!(s.col_count() * 2, sa.col_count());
	assert_eq!(sa.as_slice(), [ // TODO: mayby use epsilon as threshold?
		1.0, 0.9767660326110599, 0.5000000000000001, -0.05830891306018805, -0.49999999999999983,
		-0.8335852134237004, -1.0, -0.8808440881331612, -0.5000000000000004, -0.0000000000000002914335439641036,
		0.5000000000000001, 0.8808440881331613, 1.0, 0.8335852134237003, 0.4999999999999998,
		0.058308913060188594, -0.4999999999999993, -0.9767660326110593, -1.0, -0.5213919054209354,
	]);
}

mod test_litio {
	use std::path::PathBuf;
	use litaudio::*;
	use litcontainers::*;
	use litdsp::*;

	#[test]
	fn test_bench() {
		let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		in_path.push("assets/test_audio.wav");
		let s: AudioDeinterleaved<f64, U1, Dynamic> = litio::read_audio(in_path.as_path()).unwrap();
		let u = resampling::resample(&s, 13, 7);
	}
}