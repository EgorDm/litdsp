use litcontainers::*;
use crate::{functions, filters, window};
use super::upfirdn;
use std::cmp::max;

#[allow(non_snake_case)]
pub fn resample<D>(s: &RowVec<f64, D>, p: usize, q: usize) -> RowVec<f64, Dynamic>
	where D: Dim
{
	let gcd = functions::gcd_t(p, q);
	let (p, q) = (p / gcd, q / gcd);

	if q == p {
		return s.transmute_dims(U1, Dynamic::new(s.col_count()), s.row_stride_dim(), s.col_stride_dim()).clone_owned();
	}
	let K = max(p, q);

	// Create Filter
	let n = 10;
	let fac = 0.5 / K as f64;
	let f = rvec!(U4, &[0., 2. * fac, 2. * fac, 1.]);
	let a = rvec!(U4, &[1., 1., 0., 0.]);
	let filter_len = (2 * n * K + 1);
	let filter_len_half =  (filter_len - 1) / 2;
	let mut filter = filters::firls(D!(filter_len - 1), f, a);
	let window = window::kaiser(D!(filter_len), 5.);
	filter *= &(window * p as f64);

	let length = s.col_count();
	let output_size = functions::quotient_ceil(length * p, q);

	// Pad the filter
	let pad_before = q - filter_len_half % q;
	let delay = (filter_len_half + pad_before) / q;
	let pad_after = max(((output_size + delay) * q) as isize - ((length - 1) * p + (pad_before + filter_len)) as isize, 0) as usize;
	let mut h = rvec_zeros!(Dynamic::new(pad_before + filter_len + pad_after); f64);
	h.slice_cols_mut(SizedRange::new(pad_before, filter.col_dim())).copy_from(&filter);

	// Resample upfirdn
	upfirdn(s, p, q, h)
}