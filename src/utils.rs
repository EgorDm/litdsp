use std::f64;
use litcontainers::*;

pub fn generate_pulse<L: Dim>(f: f64, length: L, offset: usize, feature_rate: f64, sine: bool) -> ContainerRM<f64, U1, L> {
	let t = ContainerRM::linspace_rows(
		U1,
		length,
		-(offset as f64),
		(length.value() - 1 - offset) as f64
	);
	let ft = t / feature_rate * (2. * f64::consts::PI * f);

	if sine {
		ft.sin()
	} else {
		ft.cos()
	}
}