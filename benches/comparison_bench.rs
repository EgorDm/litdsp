#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::path::PathBuf;
use litaudio::*;
use litcontainers::*;
use litdsp::*;

fn criterion_benchmark(c: &mut Criterion) {
	let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	in_path.push( "assets/test_audio.wav");
	let audio: AudioDeinterleaved<f64, U1, Dynamic> = litio::read_audio(in_path.as_path()).unwrap();

	// My C++ implementation gets here about 22ms. Rust impl gets 17 ms (multithread) and (40 ms single thread). very strange
	c.bench_function("resample", move |b| b.iter(|| {
		let s = &audio;
		resampling::resample(s, s.sample_rate() as usize, (s.sample_rate() / 2) as usize);
	}));

	// TODO: optimize
	let audio: AudioDeinterleaved<f64, U1, Dynamic> = litio::read_audio(in_path.as_path()).unwrap();
	let w = RowVec::from_value(U1, D!(audio.sample_rate() as usize), 1.);
	let hop_dim = D!((audio.sample_rate() / 4) as usize);
	c.bench_function("stft", move |b| b.iter(|| {
		let s = &audio;
		stft::calculate_stft(s, w.clone_owned(), hop_dim, true, s.sample_rate() as f64);
	}));


	// TODO: optimize
	let audio: AudioDeinterleaved<f64, U1, Dynamic> = litio::read_audio(in_path.as_path()).unwrap();
	let w = RowVec::from_value(U1, D!(audio.sample_rate() as usize), 1.);
	let hop_dim = D!((audio.sample_rate() / 4) as usize);
	let freqs = RowVec::regspace_rows(U1, D!(30), 80.) * 60.;
	c.bench_function("compute_fourier_coefficients", move |b| b.iter(|| {
		let s = &audio;
		stft::compute_fourier_coefficients(s, w.clone_owned(), hop_dim, &freqs, s.sample_rate() as f64);
	}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);