use litcontainers::*;

type PaddedDim<D, PL, PR> = <<D as DimAdd<PL>>::Output as DimAdd<PR>>::Output;

pub fn pad_cols<T, S, PL, PR>(s: &S, pad_left: PL, pad_right: PR, repeat: bool)
	-> ContainerRM<T, S::Rows, PaddedDim<S::Cols, PL, PR>>
	where T: Scalar, S: Storage<T>,
	      PL: Dim, PR: Dim,
	      S::Cols: DimAdd<PL>, <S::Cols as DimAdd<PL>>::Output: DimAdd<PR>
{
	let mut ret = ContainerRM::zeros(Size::new(
		s.row_dim(),
		s.col_dim().add(pad_left).add(pad_right)
	));
	ret.slice_cols_mut(pad_left.value()..pad_left.value() + s.cols()).copy_from(s);

	if repeat {
		if pad_left.value() > 0 {
			let values = s.slice_cols(0);
			for mut col in ret.slice_cols_mut(0..pad_left.value()).as_col_slice_iter_mut() {
				col.copy_from(&values);
			}
		}

		if pad_right.value() > 0 {
			let values = s.slice_cols(s.cols() - 1);
			for mut col in ret.slice_cols_mut(ret.cols() - pad_left.value()..ret.cols()).as_col_slice_iter_mut() {
				col.copy_from(&values);
			}
		}
	}

	ret
}

pub fn pad_rows<T, S, PL, PR>(s: &S, pad_left: PL, pad_right: PR, repeat: bool)
	-> ContainerRM<T, PaddedDim<S::Rows, PL, PR>, S::Cols>
	where T: Scalar, S: Storage<T>,
	      PL: Dim, PR: Dim,
	      S::Rows: DimAdd<PL>, <S::Rows as DimAdd<PL>>::Output: DimAdd<PR>
{
	let mut ret = ContainerRM::zeros(Size::new(
		s.row_dim().add(pad_left).add(pad_right),
		s.col_dim()
	));
	ret.slice_rows_mut(pad_left.value()..pad_left.value() + s.rows()).copy_from(s);

	if repeat {
		if pad_left.value() > 0 {
			let values = s.slice_rows(0);
			for mut col in ret.slice_rows_mut(0..pad_left.value()).as_row_slice_iter_mut() {
				col.copy_from(&values);
			}
		}

		if pad_right.value() > 0 {
			let values = s.slice_rows(s.cols() - 1);
			for mut col in ret.slice_rows_mut(ret.rows() - pad_left.value()..ret.rows()).as_row_slice_iter_mut() {
				col.copy_from(&values);
			}
		}
	}

	ret
}