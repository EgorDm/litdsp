use litcontainers::*;
use litdsp::functions;

#[test]
fn interp1_nearest() {
	let in_axis = cvec!(&[1., 2., 3., 4., 5., 6.]);
	let in_values = cvec!(&[1., 2., 3., 4., 5., 6.]);
	let out_axis = cvec!(&[2., 2.5, 3., 4.9, 5.1, 5.2]);
	let mut out_values = cvec!(&[1., 2., 3., 4., 5., 6.]);

	functions::interp1_nearest_cols(&in_axis, &in_values, &out_axis, &mut out_values);
	assert_eq!(out_values.as_slice(), [2., 2., 3., 5., 5., 5.]);
}