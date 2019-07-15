use litcontainers::*;
use crate::{WindowedColIter, WindowedIter, WindowedIterMut};
use fftw::plan::*;
use fftw::types::Flag;
use std::f64;

// TODO: use size hinting at more places
pub fn calculate_stft<C, S, W, H>(signal: S, window: ContainerRM<f64, U1, W>, hop_size: H, pad: bool, sr: f64)
	-> (ContainerCM<c64, <<W as DimNameDiv<U2>>::Output as DimAdd<U1>>::Output, Dynamic>, f64)
	where C: Dim, H: Dim, S: Storage<f64, U1, C>,
	      W: Dim + DimNameDiv<U2>,
	      <W as DimNameDiv<U2>>::Output: DimAdd<U1>
{
	let window_dim = window.col_dim();
	let half_window_dim = <W as DimNameDiv<U2>>::div(window.col_dim(), U2).add(U1);

	let padding = if pad { window_dim.value() / 2 } else { 0 };
	let mut window_iter = WindowedColIter::new_padded(&signal, window.col_dim(), hop_size, padding, padding);
	let mut S = ContainerCM::zeros(half_window_dim, Dynamic::new(window_iter.window_count()));
	let mut plan = R2CPlan64::aligned(&[window_dim.value()], Flag::Estimate).unwrap();

	let mut cursor = 0;
	while let Some(mut w) = window_iter.next_window_mut() {
		w *= &window;

		let mut wS = S.slice_cols_mut(cursor);
		plan.r2c(w.as_mut_slice(), wS.as_mut_slice()).unwrap();

		cursor += 1;
	}

	(S, sr / hop_size.value() as f64)
}

pub fn calculate_time<D: Dim>(size: D, sr: f64) -> ContainerRM<f64, U1, D> {
	ContainerRM::regspace_rows(U1, size, 0.) / sr
}

pub fn calculate_freq<W>(window_size: W) -> ContainerRM<f64, U1, <<W as DimNameDiv<U2>>::Output as DimAdd<U1>>::Output>
	where W: Dim + DimNameDiv<U2>, <W as DimNameDiv<U2>>::Output: DimAdd<U1>
{
	let half_window_dim = <W as DimNameDiv<U2>>::div(window_size, U2).add(U1);
	ContainerRM::regspace_rows(U1, half_window_dim, 0.) / 2.
}

pub fn freq_index(f: f64) -> usize { (f * 2.).round() as usize }

pub fn compute_fourier_coefficients<C, S, W, H>(signal: S, window: ContainerRM<f64, U1, W>, hop_dim: H, freqs: Vec<f64>, sr: f64)
	-> ContainerCM<c64, Dynamic, Dynamic>
	where C: Dim, H: Dim, S: Storage<f64, U1, C>,
	      W: Dim + DimNameDiv<U2>
{
	let window_dim = window.col_dim();
	let overlap = window_dim.value() - hop_dim.value();

	let two_pi_t = ContainerRM::regspace_rows(U1, window_dim, 0.) * (f64::consts::PI * 2. / sr);
	let window_count = (signal.col_count() - overlap) / (window_dim.value() - overlap);

	let mut S = ContainerCM::zeros(Dynamic::new(freqs.len()), Dynamic::new(window_count));

	S.as_row_slice_par_mut_iter()
		.zip(freqs.clone())
		.for_each(|(mut row, f)| {
			let two_pi_ft = &two_pi_t * f;
			let cosine = (&two_pi_ft).cos();
			let sine = (&two_pi_ft).sin();

			let mut window_iter = WindowedColIter::new(&signal, window.col_dim(), hop_dim);
			let mut wi = 0;
			while let Some(mut w) = window_iter.next_window_mut() {
				w *= &window;
				let co = (&w * &cosine).sum(); // TODO: implement streaming for more mem efficiency
				let si = (&w * &sine).sum();

				*row.get_mut(0, wi) = c64::new(co, si);
				wi += 1;
			}
		});
	S
}