use litcontainers::*;
use crate::{WindowedColIter, WindowedIter, WindowedIterMut};
use fftw::plan::*;
use fftw::types::Flag;
use std::ops::Div;
use std::f64;
use num_traits::Float;
use num_traits::real::Real;

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