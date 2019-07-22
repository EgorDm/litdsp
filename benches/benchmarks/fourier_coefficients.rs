use criterion::Criterion;
use crate::helpers::setup_audio;
use litaudio::AudioStorage;
use litcontainers::*;
use litdsp::stft;

// TODO: optimize. Goal 397ms -> 297ms
fn compute_fourier_coefficients_benchmark(c: &mut Criterion) {
	let audio = setup_audio();

	let w = RowVec::from_value(U1, D!(audio.sample_rate() as usize), 1.);
	let hop_dim = D!((audio.sample_rate() / 2) as usize);
	let freqs = RowVec::regspace_rows(U1, D!(30), 80.) * 60.;
	c.bench_function("compute_fourier_coefficients", move |b| b.iter(|| {
		let s = &audio;
		stft::compute_fourier_coefficients(s, w.clone_owned(), hop_dim, &freqs, s.sample_rate() as f64);
	}));
}

criterion_group!{
    name = benchmark;
    config = Criterion::default().sample_size(10);
    targets = compute_fourier_coefficients_benchmark
}