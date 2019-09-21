use criterion::Criterion;
use crate::helpers::setup_audio;
use litaudio::AudioStorage;
use litcontainers::*;
use litdsp::algorithms;
use typenum::consts::U2048;

fn time_scaling_benchmark(c: &mut Criterion) {
	let audio = setup_audio();

	c.bench_function("time_scaling", move |b| b.iter(|| {
		let s = &audio;
		algorithms::calculate_pv(s, s.sample_rate() as f64, 2., U2048::name());
	}));
}

criterion_group!{
    name = benchmark;
    config = Criterion::default().sample_size(10);
    targets = time_scaling_benchmark
}