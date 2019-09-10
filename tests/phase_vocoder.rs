use litcontainers::*;
use litdsp::*;
use litaudio::*;
use std::path::PathBuf;
use typenum::consts::U2048;

pub fn setup_audio() -> AudioDeinterleaved<f64, U1, Dynamic> {
	let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/test_audio.wav");
	litaudioio::read_audio(in_path.as_path()).unwrap()
}

#[test]
pub fn phase_vocoder() {
	let freq = 3.;
	let sr = 8.;
	let window_size = U8;
	let s = wave::generate_wave(freq, U40, 0, sr, false);

	println!("{}", s);

	let result = algorithms::calculate_pv(&s, sr, 1.2, window_size);
	let result = result.into_audio(sr as i32 / 2, Deinterleaved);

	let target = [
		0.0, 0., -0., -0., 1.0606601717798212, -0.9267766952966365, -0.0000000000000005289420894161012,
		1.0000000000000007, -1.414213562373095, 0.999999999999998, 0.0000000000000025156727258250756,
		-1.0000000000000018, 1.4142135623730947, -0.9999999999999952, -0.000000000000008756730743007312,
		1.0000000000000047, -1.4142135623730951, 0.9999999999999933, 0.000000000000005026443033520969,
		-1.0000000000000049, 1.4142135623730947, -0.9999999999999936, -0.00000000000000945745443493447,
		1.0000000000000089, -1.4142135623730951, 0.9999999999999946, 0.000000000000009884932650062121,
		-1.0000000000000095, 1.4142135623730947, -0.9999999999999873, -0.000000000000017159231363308576,
		1.000000000000014, -1.414213562373095, 0.9999999999999825, 0.000000000000016969940008082114,
		-1.0000000000000229, 1.4142135623730947, -0.9999999999999871, -0.000000000000027629574319107226,
		1.0000000000000127, -1.4142135623730945, 0.9999999999999881, 0.000000000000025029285237642206,
		-0.9267766952966462, -0.0000000000000008537832885258359, 0.0000000000000032386339533442906,
		-0.0000000000000018548292789939988, 0.00000000000000027038176704884255,
	];

	for (r, t) in result.iter().zip(target.iter()) {
		assert!((r - t).abs() < 0.00001)
	}
}

#[test]
pub fn phase_vocoder_audio() {
	let audio = setup_audio();

	let result = algorithms::calculate_pv(&audio, audio.sample_rate() as f64, 2., U2048::name());
	let res = result.into_audio(audio.sample_rate() / 2, Deinterleaved);

	let target: ContainerRM<f64, U1, Dynamic> = litio::read_binary_file(
		&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/test_audio_vc_a2.lit")
	).unwrap();

	res.foreach_zip(target.iter(), |a, b| assert_eq!(*a, b));
}