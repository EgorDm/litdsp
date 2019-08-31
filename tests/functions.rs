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

#[test]
fn conv2() {
	//w.slice_mut(1..3, 1..3).fill(1.);
	let w = ContainerRM::from_vec(Size::new(U4, U4), &[
		0., 0., 0., 0.,
		0., 1., 1., 0.,
		0., 1., 1., 0.,
		0., 0., 0., 0.
	]);
	let g = rvec!(&[1., 1.]);

	let c = functions::conv2_full(&w, &g);
	assert_eq!(c.as_slice(), &[
		0., 0., 0., 0., 0.,
		0., 1., 2., 1., 0.,
		0., 1., 2., 1., 0.,
		0., 0., 0., 0., 0.
	]);

	let c = functions::conv2_same(&w, &g);
	assert_eq!(c.as_slice(), &[
		0., 0., 0., 0.,
		1., 2., 1., 0.,
		1., 2., 1., 0.,
		0., 0., 0., 0.
	]);
}

#[test]
fn padding() {
	let s = ContainerRM::regspace(Size::new(U3, U3), RowAxis, 1);

	assert_eq!(functions::pad_cols(&s, U2, U2, true).as_slice(), &[
		1, 1, 1, 2, 3, 3, 3,
		1, 1, 1, 2, 3, 3, 3,
		1, 1, 1, 2, 3, 3, 3,
	]);

	assert_eq!(functions::pad_cols(&s, U2, U2, false).as_slice(), &[
		0, 0, 1, 2, 3, 0, 0,
		0, 0, 1, 2, 3, 0, 0,
		0, 0, 1, 2, 3, 0, 0,
	]);

	assert_eq!(functions::pad_rows(&s, U2, U2, true).as_slice(), &[
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
	]);

	assert_eq!(functions::pad_rows(&s, U2, U2, false).as_slice(), &[
		0, 0, 0,
		0, 0, 0,
		1, 2, 3,
		1, 2, 3,
		1, 2, 3,
		0, 0, 0,
		0, 0, 0,
	]);
}