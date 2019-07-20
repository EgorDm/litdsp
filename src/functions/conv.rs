use litcontainers::*;

type ConvDim<W, G> = <<W as DimAdd<G>>::Output as DimSub<U1>>::Output;

/// Performs a 2D convolution in full mode
/// The output is of size D(w) + D(g) - 1
///
/// # Arguments
/// * `w` - image or other struture
/// * `g` - kernel
pub fn conv2_full<T, WR, WC, W, GR, GC, G>(w: &W, g: &G)
	-> ContainerRM<T, ConvDim<WR, GR>, ConvDim<WC, GC>>
	where T: Scalar,
	      WR: Dim + DimAdd<GR>, WC: Dim + DimAdd<GC>, W: Storage<T, WR, WC>,
	      GR: Dim, GC: Dim, G: Storage<T, GR, GC>,
	      <WR as DimAdd<GR>>::Output: DimSub<U1>,
	      <WC as DimAdd<GC>>::Output: DimSub<U1>
{
	let out_rows = w.row_dim().add(g.row_dim()).sub(U1);
	let out_cols = w.col_dim().add(g.col_dim()).sub(U1);

	let h = g.flip();

	let mut x = ContainerRM::zeros(D!(w.row_count() + 2 * (h.row_count() - 1)), D!(w.col_count() + 2 * (h.col_count() - 1)));
	x.slice_mut(SizedRange::new(h.row_count() - 1, w.row_dim()), SizedRange::new(h.col_count() - 1, w.col_dim()))
		.copy_from(w);

	let mut out = ContainerRM::zeros(out_rows, out_cols);

	for (row, mut row_slice) in out.as_row_slice_mut_iter().enumerate() {
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
pub fn conv2_same<T, WR, WC, W, GR, GC, G>(w: &W, g: &G)
	-> ContainerRM<T, WR, WC>
	where T: Scalar,
	      WR: Dim + DimAdd<GR>, WC: Dim + DimAdd<GC>, W: Storage<T, WR, WC>,
	      GR: Dim, GC: Dim, G: Storage<T, GR, GC>,
	      <WR as DimAdd<GR>>::Output: DimSub<U1>,
	      <WC as DimAdd<GC>>::Output: DimSub<U1>
{
	let ret = conv2_full(w, g);
	let start_row = g.row_count() / 2;
	let start_col = g.col_count() / 2;
	ret.slice(SizedRange::new(start_row, w.row_dim()), SizedRange::new(start_col, w.col_dim())).clone_owned()
}
