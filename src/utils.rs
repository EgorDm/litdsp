use std::f64;
use litcontainers::*;

pub fn generate_pulse<L: Dim>(f: f64, length: L, offset: usize, feature_rate: usize) -> ContainerRM<f64, U1, L> {
	let t = ContainerRM::linspace_rows(
		U1,
		length,
		-(offset as f64),
		(length.value() - 1 - offset) as f64
	);
	let ft = t / feature_rate as f64 * (2. * f64::consts::PI * f);
	ft.sin()
}