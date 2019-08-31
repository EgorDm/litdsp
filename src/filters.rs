use litcontainers::*;
use crate::functions::*;
use itertools::Itertools;
use std::f64::consts::PI;

const PI2: f64 = 2. * PI;

/// Least-squares linear-phase FIR filter design [Usage](https://www.mathworks.com/help/signal/ref/firls.html)
/// # Arguments
/// * `n` - Filter length
/// * `f` - Vector of pairs of frequency points, specified in the range between 0 and 1, where 1 corresponds to the Nyquist frequency.
/// * `a` - Vector containing the desired amplitude at the points specified in f
/// https://cnx.org/contents/6x7LNQOp@7/Linear-Phase-Fir-Filter-Design-By-Least-Squares
pub fn firls<D, F>(l: D, mut f: RowVec<f64, F>, a: RowVec<f64, F>)
	-> RowVec<f64, <D as DimAdd<U1>>::Output>
	where F: Dim,
	      D: Dim + DimAdd<U1> + DimDiv<U2>,
			<D as DimDiv<U2>>::Output: DimAdd<U1>
{
	assert_eq!(f.size(), a.size(), "Both f and a should be same size");
	f /= 2.;
	let n = l.add(U1);
	let m = l.div(U2);
	let is_odd = (n.value() % 2) == 1;

	let k_dim = m.add(U1);
	let k = RowVec::regspace(Size::new(U1, k_dim.clone()), RowAxis, if !is_odd { 0.5 } else { 0. });

	// Differentiate
	let mut b0 = 0.;
	let mut b = RowVec::from_value(Size::new(U1, k_dim), 0.);
	for ((f1, a1), (f2, a2)) in f.as_iter().zip(a.as_iter()).tuples() {
		let freq_diff = f2 - f1;
		let dw = (a2 - a1) / freq_diff;
		let b1 = a1 - dw * f1;

		if is_odd {
			b0 += (b1 * freq_diff) + dw / 2. * (f2 * f2 - f1 * f1);
		}

		// TODO: implement as container ops instead? Higher chance on vectorization?
		for (i, bi) in b.as_iter_mut().enumerate() {
			*bi += dw / (4. * PI * PI) * ((PI2 * k[i] * f2).cos() - (PI2 * k[i] * f1).cos()) / (k[i] * k[i]);
			*bi += (f2 * (dw * f2 + b1) * sinc(2. * k[i] * f2))
				- (f1 * (dw * f1 + b1) * sinc(2. * k[i] * f1));
		}
	}

	if is_odd { b[0] = b0; }
	let a = (b * 4.) / 2.;

	//let ret = RowVec::zeros(Size::new(U1, n));
	if is_odd {
		join_cols!(&a.flip(), &a.slice_cols(1..a.cols()); n)
		//unsafe { ret.join_cols_unchecked(&a.flip(), &a.slice_cols(1..a.col_count()) }
	} else {
		//unsafe { ret.join_cols_unchecked(&a.flip(), &a) }
		join_cols!(&a.flip(), &a; n)
	}
}