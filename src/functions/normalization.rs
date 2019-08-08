use litcontainers::*;
use num_traits::{Signed};

pub fn normalize_cols_p1_inplace<T, R, C, S>(s: &mut S)
	where T: Scalar + Signed, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	normalize_cols_inplace(s, |s| norm_p1(s))
}

pub fn normalize_rows_p1_inplace<T, R, C, S>(s: &mut S)
	where T: Scalar + Signed, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	normalize_rows_inplace(s, |s| norm_p1(s))
}

pub fn normalize_cols_inplace<T, R, C, S, F>(s: &mut S, norm: F)
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>,
	      F: Fn(&SliceMut<T, R, S::RStride, U1, S::CStride>) -> T
{
	s.as_col_slice_mut_iter().for_each(|mut col| col /= norm(&col));
}

pub fn normalize_rows_inplace<T, R, C, S, F>(s: &mut S, norm: F)
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>,
		  F: Fn(&SliceMut<T, U1, S::RStride, C, S::CStride>) -> T
{
	s.as_row_slice_mut_iter().for_each(|mut row| row /= norm(&row));
}

