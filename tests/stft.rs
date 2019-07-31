#![allow(non_snake_case)]

use litcontainers::*;
use litdsp::*;

#[allow(non_snake_case)]
#[test]
fn stft() {
	let freq = 1.;
	let fr = 6.;
	let s = wave::generate_wave(freq, U40, 0, fr, false);
	let w = ContainerRM::from_value(U1, U12, 1.);
	let (S, sr) = stft::calculate_stft(&s, &w, U6, true, fr);
	let m = S.norm();

	let f = stft::calculate_freq(w.col_dim());
	let fi = stft::freq_index(freq);

	let freq_intens = m.slice(fi, 1..S.col_count() - 1).mean();
	assert_eq!(freq_intens.round() as i32, (w.sum().round() / 2.) as i32);
	assert_eq!(sr, 1.);
	assert_eq!(f.as_slice(), [0.0, 0.5, 1., 1.5, 2.,  2.5, 3.]);

//	println!("{}", f);
//	println!("{}", t);
//	println!("{}", m);
}

#[allow(non_snake_case)]

#[test]
fn compute_fourier_coefficients() {
	let freq = 1.;
	let fr = 6.;
	let s = wave::generate_wave(freq, U40, 0, fr, false);
	let w = ContainerRM::from_value(U1, U12, 1.);
	let f = RowVec::regspace_rows(U1, U7 , 0.) / 2. ;
	let (S, _sr) = stft::calculate_fourier_coefficients(&s, &w, U6, &f, fr);
	let m = S.norm();

	let fi = stft::freq_index(freq);
	let freq_intens = m.slice(fi, 1..S.col_count() - 1).mean();
	assert_eq!(freq_intens.round() as i32, (w.sum().round() / 2.) as i32);

	//println!("{}", m);
}