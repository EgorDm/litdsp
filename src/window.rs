use litcontainers::*;
use std::f64;
use crate::functions::*;

/// Hanning window
/// # Arguments
/// * `size` - Nr of taps
pub fn hanning<D: Dim>(size: D) -> RowVec<f64, D> {
	let mut w = RowVec::zeros(U1, size);
	let n = (size.value() - 1) as f64;
	for i in 0..size.value() {
		*w.get_mut_at(i) = 0.5 - 0.5 * (f64::consts::PI * 2. * i as f64 / n).cos();
	}
	w
}

/// Hamming window
/// # Arguments
/// * `size` - Nr of taps
pub fn hamming<D: Dim>(size: D) -> RowVec<f64, D> {
	let mut w = RowVec::zeros(U1, size);
	let n = (size.value() - 1) as f64;
	for i in 0..size.value() {
		*w.get_mut_at(i) = 0.54 - 0.46 * (f64::consts::PI * 2. * i as f64 / n).cos();
	}
	w
}

/// Sin window
pub fn sinw<D: Dim>(size: D, beta: f64) -> RowVec<f64, D> {
	let w = RowVec::regspace_rows(U1, size, 0.);
	(w * (f64::consts::PI / size.value() as f64)).sin().pow(beta)
}

/// Kaiser window.
/// Credit: Sigpack
/// See Kaiser window at [Wikipedia](https://en.wikipedia.org/wiki/Window_function#Kaiser_window)
///
/// # Arguments
/// * `size` - Nr of taps
/// * `beta` - Beta factor
#[allow(non_snake_case)]
pub fn kaiser<D: Dim>(size: D, beta: f64) -> RowVec<f64, D> {
	let mut h = RowVec::zeros(U1, size);
	let bb = besseli0(beta);
	let N  = size.value();
	for (i, x) in h.as_iter_mut().enumerate() {
		*x = besseli0(beta * (4.0 * i as f64 * (N - 1 - i) as f64).sqrt() / (N - 1) as f64) / bb
	}
	h
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
