use litcontainers::*;

type ConvDim<W, G> = <<W as DimAdd<G>>::Output as DimSub<U1>>::Output;

/// Performs a 2D convolution in full mode
/// The output is of size D(w) + D(g) - 1
///
/// # Arguments
/// * `w` - image or other struture
/// * `g` - kernel
pub fn conv2_full<T, W, G>(w: &W, g: &G)
	-> ContainerRM<T, ConvDim<W::Rows, G::Rows>, ConvDim<W::Cols, G::Cols>>
	where T: Scalar,
	      G: Storage<T>,
	      W: Storage<T>, W::Rows: Dim + DimAdd<G::Rows>, W::Cols: Dim + DimAdd<G::Cols>,
	      <W::Rows as DimAdd<G::Rows>>::Output: DimSub<U1>,
	      <W::Cols as DimAdd<G::Cols>>::Output: DimSub<U1>
{
	let out_rows = w.row_dim().add(g.row_dim()).sub(U1);
	let out_cols = w.col_dim().add(g.col_dim()).sub(U1);

	let h = g.flip();

	let mut x = ContainerRM::zeros(Size::new(D!(w.rows() + 2 * (h.rows() - 1)), D!(w.cols() + 2 * (h.cols() - 1))));
	x.slice_mut(SizedRange::new(h.rows() - 1, w.row_dim()), SizedRange::new(h.cols() - 1, w.col_dim()))
		.copy_from(w);

	let mut out = ContainerRM::zeros(Size::new(out_rows, out_cols));

	for (row, mut row_slice) in out.as_row_slice_iter_mut().enumerate() {
		for (col, out_val) in row_slice.as_iter_mut().enumerate() {
			*out_val = x.slice(SizedRange::new(row, h.row_dim()), SizedRange::new(col, h.col_dim()))
				.iter().zip(h.as_iter()).map(|(xe, he)| xe * *he).sum();
		}
	}

	out
}

/// Performs a 2D convolution in same mode
/// The output has the same size als the original image or input
///
/// # Arguments
/// * `w` - image or other struture
/// * `g` - kernel
pub fn conv2_same<T, W, G>(w: &W, g: &G)
	-> ContainerRM<T, W::Rows, W::Cols>
	where T: Scalar,
	      G: Storage<T>,
	      W: Storage<T>, W::Rows: Dim + DimAdd<G::Rows>, W::Cols: Dim + DimAdd<G::Cols>,
	      <W::Rows as DimAdd<G::Rows>>::Output: DimSub<U1>,
	      <W::Cols as DimAdd<G::Cols>>::Output: DimSub<U1>
{
	let ret = conv2_full(w, g);
	let start_row = g.rows() / 2;
	let start_col = g.cols() / 2;
	ret.slice(SizedRange::new(start_row, w.row_dim()), SizedRange::new(start_col, w.col_dim())).clone_owned()
}
