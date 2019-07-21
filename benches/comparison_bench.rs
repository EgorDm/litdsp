#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use std::path::PathBuf;
use litaudio::*;
use litcontainers::*;
use litdsp::*;
use std::rc::Rc;

fn criterion_benchmark(c: &mut Criterion) {
	let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	in_path.push( "assets/test_audio.wav");
	let audio: AudioDeinterleaved<f64, U1, Dynamic> = litio::read_audio(in_path.as_path()).unwrap();

	// My C++ implementation gets here about 22ms. Rust impl gets 17 ms (multithread) and (40 ms single thread). very strange
	c.bench_function("resample", move |b| b.iter(|| {
		let s = &audio;
		resampling::resample(s, s.sample_rate() as usize, (s.sample_rate() / 2) as usize);
	}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);