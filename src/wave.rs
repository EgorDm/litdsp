use std::f64;
use litcontainers::*;

pub fn generate_wave<L: Dim>(f: f64, length: L, offset: usize, sr: f64, sine: bool) -> ContainerRM<f64, U1, L> {
	let t = ContainerRM::linspace_rows(
		U1,
		length,
		-(offset as f64),
		(length.value() - 1 - offset) as f64
	);
	let ft = t / sr * (2. * f64::consts::PI * f);

	if sine {
		ft.sin()
	} else {
		ft.cos()
	}
}

pub fn calculate_time<D: Dim>(size: D, sr: f64) -> ContainerRM<f64, U1, D> {
	ContainerRM::regspace_rows(U1, size, 0.) / sr
}