use litcontainers::*;
use crate::functions::*;
use itertools::Itertools;
use std::f64::consts::PI;

const PI2: f64 = 2. * PI;

type Sub1Div2<D> = <<D as DimSub<U1>>::Output as DimDiv<U2>>::Output;

/// Least-squares linear-phase FIR filter designcollapse
/// # Arguments
/// * `n` - Filter length
/// * `f` - Vector of pairs of frequency points, specified in the range between 0 and 1, where 1 corresponds to the Nyquist frequency.
/// * `a` - Cector containing the desired amplitude at the points specified in f
/// https://cnx.org/contents/6x7LNQOp@7/Linear-Phase-Fir-Filter-Design-By-Least-Squares
pub fn firls<D, F>(n: D, f: RowVec<f64, F>, a: RowVec<f64, F>)
	-> RowVec<f64, Sub1Div2<D>>
	where F: Dim,
	      D: Dim + DimSub<U1> + DimDiv<U2>,
	      <D as DimSub<U1>>::Output: DimDiv<U2>,
	      Sub1Div2<D>: DimAdd<U1>,
{
	assert_eq!(f.size(), a.size(), "Both f and a should be same size");
	let m: Sub1Div2<D> = n.sub(U1).div(U2);
	let is_odd = (m.value() % 2) == 1;

	let k_dim = m.add(U1);
	let k = RowVec::regspace_rows(U1, k_dim.clone(), if !is_odd { 0.5 } else { 0. });

	// Differentiate
	let mut b0 = 0.;
	let mut b = RowVec::from_value(U1, k_dim, 0.);
	for ((f1, a1), (f2, a2)) in f.as_iter().zip(a.as_iter()).tuple_windows() {
		let freq_diff = f2 - f1;
		let dw = (a2 - a1) / freq_diff;
		let b1 = a1 - dw * f1;

		if is_odd {
			b0 += (b1 * f1) + dw / 2. * (f2 * f2 - f1 * f1);
		}

		for (i, bi) in b.as_iter_mut().enumerate() {
			*bi += dw / (4. * PI * PI) * ((PI2 * k[i] * f2).cos() - (PI2 * k[i] * f1).cos()) / (k[i] * k[i]);
			*bi += (f2 * (dw * f2 + b1) * sinc(2. * k[i] * f2))
				- (f1 * (dw * f1 + b1) * sinc(2. * k[i] * f1));
		}
	}

	if is_odd { b[0] = b0; }
	let a = (b * 4.) / 2.;

	// TODO: flip and join a
	unimplemented!()
}