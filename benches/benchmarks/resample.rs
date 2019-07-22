use criterion::Criterion;
use crate::helpers::setup_audio;
use litaudio::AudioStorage;
use litdsp::resampling;

// My C++ implementation gets here about 22ms. Rust impl gets 17 ms (multithread) and (40 ms single thread). very strange
fn resample_benchmark(c: &mut Criterion) {
	let audio = setup_audio();

	c.bench_function("resample", move |b| b.iter(|| {
		let s = &audio;
		resampling::resample(s, s.sample_rate() as usize, (s.sample_rate() / 2) as usize);
	}));
}

criterion_group!{
    name = benchmark;
    config = Criterion::default().sample_size(40);
    targets = resample_benchmark
}