use criterion::Criterion;
use crate::helpers::setup_audio;
use litaudio::AudioStorage;
use litcontainers::*;
use litdsp::stft;

// TODO: optimize. Goal 21.96ms -> lower. Already better than c++
fn stft_benchmark(c: &mut Criterion) {
	let audio = setup_audio();
	let w = RowVec::from_value(U1, D!(audio.sample_rate() as usize), 1.);
	let hop_dim = D!((audio.sample_rate() / 2) as usize);
	c.bench_function("stft", move |b| b.iter(|| {
		let s = &audio;
		stft::calculate_stft(s, w.clone_owned(), hop_dim, true, s.sample_rate() as f64);
	}));
}

criterion_group!{
    name = benchmark;
    config = Criterion::default().sample_size(30);
    targets = stft_benchmark
}