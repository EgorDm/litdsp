use litcontainers::*;
use std::f64;

pub fn hanning_f64<W: Dim>(window_size: W) -> ContainerRM<f64, U1, W> {
	let mut w = ContainerRM::zeros(U1, window_size);
	let n = (window_size.value() - 1) as f64;
	for i in 0..window_size.value() {
		*w.get_mut_at(i) = 0.5 * (1. - (f64::consts::PI * 2. * i as f64 / n).cos());
	}
	w
}

pub fn sinw_f64<W: Dim>(window_size: W, beta: f64) -> ContainerRM<f64, U1, W> {
	let w = ContainerRM::regspace_rows(U1, window_size, 0.);
	(w * (f64::consts::PI / window_size.value() as f64)).sin().pow(beta)
}

/*
My attempt at a generic window creation which doent look really good and is overengineered
pub fn hanning<T: Scalar + Float, W: Dim>(window_size: W) -> ContainerRM<T, U1, W> {
	let mut w = ContainerRM::zeros(U1, window_size);
	let pi: T = cast(f64::consts::PI).unwrap();
	let ws: T = cast((window_size.value() - 1)).unwrap();
	for i in 0..window_size.value() {
		let u = (pi * cast(i).unwrap()).cos() / ws;
		//w[i] = 0.5 as T - 0.5 as T * std::cos((constants::PI2 * i) / (n - 1));
	}
	w
}*/
