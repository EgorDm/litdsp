use litcontainers::*;
use litdsp::*;

#[test]
fn generate_pulse() {
	let w = utils::generate_pulse(1., U20, 0, 20., false);
	dbg!(w.as_slice());
}

#[test]
fn stft() {
	let freq = 1.;
	let fr = 6.;
	let s = utils::generate_pulse(freq, U40, 0, fr, false);
	let w = ContainerRM::from_value(U1, U12, 1.);
	let (S, sr) = stft::calculate_stft(s, w.clone_owned(), U6, true, fr);
	let m = S.norm();

	let mut f = stft::calculate_freq(w.col_dim());
	let mut t = stft::calculate_time(S.col_dim(), sr);
	let fi = stft::freq_index(freq);

	let freq_intens = S.slice(fi, 1..S.col_count() - 1).norm().mean();
	assert_eq!(freq_intens.round() as i32, (w.sum().round() / 2.) as i32);
	assert_eq!(t.as_slice(), [0., 1., 2., 3., 4., 5., 6.]);
	assert_eq!(f.as_slice(), [0.0, 0.5, 1., 1.5, 2.,  2.5, 3.]);

//	println!("{}", f);
//	println!("{}", t);
//	println!("{}", m);
}


#[test]
fn compute_fourier_coefficients() {
	let freq = 1.;
	let fr = 6.;
	let s = utils::generate_pulse(freq, U40, 0, fr, false);
	let w = ContainerRM::from_value(U1, U12, 1.);
	let f = (0..7).map(|x| x as f64 / 2.).collect();
	let S = stft::compute_fourier_coefficients(s, w.clone_owned(), U6, f, fr);
	let m = S.norm();

	let fi = stft::freq_index(freq);
	let freq_intens = S.slice(fi, 1..S.col_count() - 1).norm().mean();
	assert_eq!(freq_intens.round() as i32, (w.sum().round() / 2.) as i32);

	println!("{}", m);
	let u = 0;
}