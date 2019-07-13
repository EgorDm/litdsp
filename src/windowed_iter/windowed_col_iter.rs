use litcontainers::*;
use std::marker::PhantomData;
use crate::windowed_iter::{WindowedIter, WindowedIterMut};
use std::cmp::{max, min};
use litcontainers::iterator::iter_tools::{assign_all, copy_all};

pub struct WindowedColIter<'a, T, C, S, W, H>
	where T: Scalar, C: Dim, S: Storage<T, U1, C>, W: Dim, H: Dim
{
	storage: &'a S,
	buffer: ContainerRM<T, U1, W>,
	window_size: W,
	hop_size: H,
	cursor: isize,
	cursor_end: usize,
	window_count: usize,
	_phantoms: PhantomData<(C)>
}

impl<'a, T, C, S, W, H> WindowedColIter<'a, T, C, S, W, H>
	where T: Scalar, C: Dim, S: Storage<T, U1, C>, W: Dim, H: Dim
{
	pub fn new(data: &'a S, window_dim: W, hop_dim: H) -> Self {
		Self::new_padded(data, window_dim, hop_dim, 0, 0)
	}

	pub fn new_padded(data: &'a S, window_dim: W, hop_dim: H, pad_left: usize, pad_right: usize) -> Self {
		let cursor = -(pad_left as isize);
		let cursor_end = data.col_dim().value() + pad_right;
		let overlap = window_dim.value() - hop_dim.value();
		let window_count = (cursor_end as isize - cursor - overlap as isize) as usize / (window_dim.value() - overlap);

		Self {
			storage: data,
			buffer: ContainerRM::zeros(U1, window_dim),
			window_size: window_dim,
			hop_size: hop_dim,
			cursor,
			cursor_end,
			window_count,
			_phantoms: PhantomData
		}
	}
}

impl<'a, T, C, S, W, H> WindowedIter<'a, T, W, H> for WindowedColIter<'a, T, C, S, W, H>
	where T: Scalar, C: Dim, S: Storage<T, U1, C>, W: Dim, H: Dim
{
	type WC = W;
	type WCS = U1;
	type WR = U1;
	type WRS = W;

	fn next_window<'at: 'b, 'b>(&'at mut self) -> Option<Slice<'b, T, Self::WR, Self::WRS, Self::WC, Self::WCS>>  {
		let start = self.cursor;
		let end = (start + self.window_size() as isize) as usize;

		if end > self.cursor_end {
			None
		} else {
			self.cursor += self.hop_size() as isize;

			let copy_start = max(start, 0) as usize;
			let copy_end = min(end, self.storage.col_count()) ;
			let pad_left = (copy_start as isize - start) as usize;
			let pad_right = end - copy_end;

			if pad_left == 0 && pad_right == 0 {
				return Some(
					self.storage.slice_cols(SizedRange::new(start as usize, self.window_dim()))
						.transmute_stride_dims_inplace(self.window_dim(), U1)
				);
			}

			if pad_left > 0 {
				assign_all(self.buffer.as_col_slice_mut_iter(0..pad_left), T::default());
			}

			if pad_right > 0 {
				assign_all(self.buffer.as_col_slice_mut_iter((self.window_size() - pad_right)..self.window_size()), T::default());
			}

			copy_all(
				self.buffer.as_col_slice_mut_iter(pad_left..(self.window_size() - pad_right)),
				self.storage.as_col_slice_iter(copy_start..copy_end)
			);

			Some(self.buffer.slice_rows(0))
		}
	}

	fn window_dim(&self) -> W { self.window_size }

	fn hop_dim(&self) -> H { self.hop_size }

	fn window_count(&self) -> usize { self.window_count } // TODO: Please change this to dim
}

impl<'a, T, C, S, W, H> WindowedIterMut<'a, T, W, H> for WindowedColIter<'a, T, C, S, W, H>
	where T: Scalar, C: Dim, S: Storage<T, U1, C>, W: Dim, H: Dim
{
	fn next_window_mut<'at: 'b, 'b>(&'at mut self) -> Option<SliceMut<'b, T, Self::WR, Self::WRS, Self::WC, Self::WCS>> {
		let start = self.cursor;
		let end = (start + self.window_size() as isize) as usize;

		if end > self.cursor_end {
			None
		} else {
			self.cursor += self.hop_size() as isize;

			let copy_start = max(start, 0) as usize;
			let copy_end = min(end, self.storage.col_count()) ;
			let pad_left = (copy_start as isize - start) as usize;
			let pad_right = end - copy_end;

			if pad_left > 0 {
				assign_all(self.buffer.as_col_slice_mut_iter(0..pad_left), T::default());
			}

			if pad_right > 0 {
				assign_all(self.buffer.as_col_slice_mut_iter((self.window_size() - pad_right)..self.window_size()), T::default());
			}

			copy_all(
				self.buffer.as_col_slice_mut_iter(pad_left..(self.window_size() - pad_right)),
				self.storage.as_col_slice_iter(copy_start..copy_end)
			);

			Some(self.buffer.slice_rows_mut(0))
		}
	}
}