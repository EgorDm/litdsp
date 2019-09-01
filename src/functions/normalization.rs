use litcontainers::*;
use num_traits::{Signed};

pub fn normalize_cols_p1_inplace<T, S>(s: &mut S)
	where T: Scalar + Signed, S: StorageMut<T>
{
	normalize_cols_inplace(s, |s| norm_p1(s))
}

pub fn normalize_rows_p1_inplace<T, S>(s: &mut S)
	where T: Scalar + Signed, S: StorageMut<T>
{
	normalize_rows_inplace(s, |s| norm_p1(s))
}

pub fn normalize_cols_inplace<T, S, F>(s: &mut S, norm: F)
	where T: NumericElement, S: StorageMut<T>,
	      F: Fn(&SliceMut<T, S::Rows, S::RowStride, U1, S::ColStride>) -> T
{
	s.as_col_slice_iter_mut().for_each(|mut col| col /= norm(&col));
}

pub fn normalize_rows_inplace<T, S, F>(s: &mut S, norm: F)
	where T: NumericElement, S: StorageMut<T>,
		  F: Fn(&SliceMut<T, U1, S::RowStride, S::Cols, S::ColStride>) -> T
{
	s.as_row_slice_iter_mut().for_each(|mut row| row /= norm(&row));
}

