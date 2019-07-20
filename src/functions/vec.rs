use litcontainers::*;
use itertools::{Itertools, MinMaxResult};
use num_traits::Signed;

pub fn interp1_nearest_cols<TA, TV, AI, AO, C>(axis_in: &ColVec<TA, AI>, values_in: &ContainerRM<TV, AI, C>, axis_out: &ColVec<TA, AO>, values_out: &mut ContainerRM<TV, AO, C>)
	where TA: ElementaryScalar + Signed, TV: Scalar, AI: Dim, AO: Dim, C: Dim
{
	assert!(axis_in.row_count() == values_in.row_count() && axis_out.row_count() == axis_out.row_count() && values_in.col_count() == values_out.col_count(), "Container dimensions are not valid");
	let (axis_in_min, axis_in_max) = match axis_in.as_col_iter().minmax() {
		MinMaxResult::NoElements => (TA::default(), TA::default()), // Should not be possible
		MinMaxResult::OneElement(v) => (*v, *v),
		MinMaxResult::MinMax(min, max) => (*min, *max),
	};

	let axis_size_in = axis_in.row_count();
	let axis_size_out = axis_out.row_count();

	let mut best_j = 0;

	for i in 0..axis_size_out {
		let mut best_err = TA::max_val();
		let axis_out_val = axis_out[i];

		if axis_out_val < axis_in_min {
			values_out.slice_rows_mut(i).copy_from(&values_in.slice_rows(0));
		} else if axis_out_val > axis_in_max {
			values_out.slice_rows_mut(i).copy_from(&values_in.slice_rows(axis_size_out - 1));
		} else {
			for j in best_j..axis_size_in {
				let err = (axis_in[j] - axis_out_val).abs();

				if err < best_err {
					best_err = err;
					best_j = j;
				} else {
					break;
				}
			}
			values_out.slice_rows_mut(i).copy_from(&values_in.slice_rows(best_j));
		}
	}
}