use litcontainers::*;

type PaddedDim<D, PL, PR> = <<D as DimAdd<PL>>::Output as DimAdd<PR>>::Output;

pub fn pad_cols<T, R, C, S, PL, PR>(s: &S, pad_left: PL, pad_right: PR, repeat: bool)
	-> ContainerRM<T, R, PaddedDim<C, PL, PR>>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
	      PL: Dim, PR: Dim,
	      C: DimAdd<PL>, <C as DimAdd<PL>>::Output: DimAdd<PR>
{
	let mut ret = ContainerRM::zeros(s.row_dim(), s.col_dim().add(pad_left).add(pad_right));
	ret.slice_cols_mut(pad_left.value()..pad_left.value() + s.col_count()).copy_from(s);

	if repeat {
		if pad_left.value() > 0 {
			let values = s.slice_cols(0);
			for mut col in ret.slice_cols_mut(0..pad_left.value()).as_col_slice_mut_iter() {
				col.copy_from(&values);
			}
		}

		if pad_right.value() > 0 {
			let values = s.slice_cols(s.col_count() - 1);
			for mut col in ret.slice_cols_mut(ret.col_count() - pad_left.value()..ret.col_count()).as_col_slice_mut_iter() {
				col.copy_from(&values);
			}
		}
	}

	ret
}

pub fn pad_rows<T, R, C, S, PL, PR>(s: &S, pad_left: PL, pad_right: PR, repeat: bool)
	-> ContainerRM<T, PaddedDim<R, PL, PR>, C>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
	      PL: Dim, PR: Dim,
	      R: DimAdd<PL>, <R as DimAdd<PL>>::Output: DimAdd<PR>
{
	let mut ret = ContainerRM::zeros(s.row_dim().add(pad_left).add(pad_right), s.col_dim());
	ret.slice_rows_mut(pad_left.value()..pad_left.value() + s.row_count()).copy_from(s);

	if repeat {
		if pad_left.value() > 0 {
			let values = s.slice_rows(0);
			for mut col in ret.slice_rows_mut(0..pad_left.value()).as_row_slice_mut_iter() {
				col.copy_from(&values);
			}
		}

		if pad_right.value() > 0 {
			let values = s.slice_rows(s.col_count() - 1);
			for mut col in ret.slice_rows_mut(ret.row_count() - pad_left.value()..ret.row_count()).as_row_slice_mut_iter() {
				col.copy_from(&values);
			}
		}
	}

	ret
}