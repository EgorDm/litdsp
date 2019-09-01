use litcontainers::*;
use crate::windowed_iter::*;
use fftw::plan::*;
use fftw::types::Flag;
use std::f64;
use rayon::prelude::*;

// TODO: can we add f32 variants?

/// Calculates Short-time Fourier transform
/// For this FFTW library is used for FFT calculation
/// Results in a container with columns containing frequency data and rows containing temporal data
/// # Arguments
/// * `s` - Given input signal
/// * `w` - window. Amount of frequency bins generated corresponds to (window_length / 2 + 1)
/// * `hop_dim` - correlates to the amount of windows taken and amount of temporal data generated
/// * `pad` - whether signal should be padded first. Padding will be window_length / 2  at negin and end of the signal
/// * `sr` - signal sampling rate
#[allow(non_snake_case)]// TODO: use size hinting at more places
pub fn calculate_stft<C, S, W, H>(s: &S, window: &ContainerRM<f64, U1, W>, hop_dim: H, pad: bool, sr: f64)
	-> (ContainerCM<c64, <<W as DimDiv<U2>>::Output as DimAdd<U1>>::Output, Dynamic>, f64)
	where C: Dim, H: Dim, S: Storage<f64> + StorageSize<Rows=U1, Cols=C>,
	      W: Dim + DimDiv<U2>,
	      <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	let window_dim = window.col_dim();
	let half_window_dim = <W as DimDiv<U2>>::div(window.col_dim(), U2).add(U1);

	let padding = if pad { window_dim.value() / 2 } else { 0 };
	let mut window_iter = WindowedColIter::new_padded(s, window.col_dim(), hop_dim, padding, padding);
	let mut S = ContainerCM::zeros(Size::new(
		half_window_dim,
		Dynamic::new(window_iter.window_count())
	));
	let mut plan = R2CPlan64::aligned(&[window_dim.value()], Flag::Estimate).unwrap();

	let mut cursor = 0; // TODO: create paralallel iter
	while let Some(mut w) = window_iter.next_window_mut() {
		w *= window;

		let mut wS = S.slice_cols_mut(cursor);
		plan.r2c(w.as_slice_mut(), wS.as_slice_mut()).unwrap();

		cursor += 1;
	}

	(S, sr / hop_dim.value() as f64)
}

pub fn calculate_freq<W>(window_size: W) -> ContainerRM<f64, U1, <<W as DimDiv<U2>>::Output as DimAdd<U1>>::Output>
	where W: Dim + DimDiv<U2>, <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	let half_window_dim = <W as DimDiv<U2>>::div(window_size, U2).add(U1);
	ContainerRM::regspace(Size::new(U1, half_window_dim), RowAxis, 0.) / 2.
}

pub fn freq_index(f: f64) -> usize { (f * 2.).round() as usize }

/// Calculates Short-time Fourier transform based on provided fequencies
/// Uses DFT algorithm for freq calculation
/// Results in a container with columns containing frequency data and rows containing temporal data
/// # Arguments
/// * `s` - Given input signal
/// * `w` - window. Amount of frequency bins generated corresponds to (window_length / 2 + 1)
/// * `hop_dim` - correlates to the amount of windows taken and amount of temporal data generated
/// * `freqs` - frequencies thet need to be sampled
/// * `sr` - signal sampling rate
#[allow(non_snake_case)]
pub fn calculate_fourier_coefficients<S, W, H, F>(s: &S, window: &ContainerRM<f64, U1, W>, hop_dim: H, freqs: &RowVec<f64, F>, sr: f64)
	-> (ContainerRM<c64, F, Dynamic>, f64)
	where H: Dim, S: RowVecStorage<f64>, F: Dim,
	      W: Dim + DimDiv<U2>
{
	let window_dim = window.col_dim();
	let overlap = window_dim.value() - hop_dim.value();

	let two_pi_t = ContainerRM::regspace(Size::new(U1, window_dim), RowAxis, 0.) * (f64::consts::PI * 2. / sr);
	let window_count = (s.cols() - overlap) / (window_dim.value() - overlap);

	let mut S = ContainerRM::zeros(Size::new(
		freqs.col_dim(),
		Dynamic::new(window_count)
	));

	S.as_row_slice_iter_mut()
		.into_par_iter()
		.zip(row_iter(freqs, 0).into_par_iter())
		.for_each(|(mut row, f)| {
			let two_pi_ft = &two_pi_t * *f;
			let cosine = (&two_pi_ft).cos();
			let sine = (&two_pi_ft).sin();

			let mut window_iter = WindowedColIter::new(s, window.col_dim(), hop_dim);
			let mut wi = 0;
			while let Some(mut w) = window_iter.next_window_mut() {
				w *= window;

				let co = w.as_iter().zip(cosine.as_iter()).map(|(a, b)| a * b).sum(); // (&w * &cosine).sum()
				let si = w.as_iter().zip(sine.as_iter()).map(|(a, b)| a * b).sum(); // (&w * &sine)

				*row.get_mut(0, wi) = c64::new(co, si);
				wi += 1;
			}
		});

	(S, sr / hop_dim.value() as f64)
}