use litcontainers::*;
use crate::functions;
use std::cmp::{min};
use num_traits::Float;
use rayon::prelude::*;


struct Upfirdn<T: Scalar + Float> {
	p: usize,
	q: usize,
	coefs_t: ContainerRM<T, Dynamic, Dynamic>,
	coefs_per_phase: usize
}

macro_rules! get_n_advance {
	($ptr: ident) => ({
		let ret = *$ptr;
		$ptr = $ptr.offset(1);
		ret
	})
}

unsafe fn get_n_advance_mut<T: Sized>(ptr: &mut *mut T) -> &mut T {
	let ret = &mut *(*ptr);
	*ptr = ptr.offset(1);
	ret
}

pub unsafe fn offset_from<T: Sized>(target: *const T, origin: *const T) -> isize {
	let pointee_size = std::mem::size_of::<T>();
	isize::wrapping_sub(target as isize, origin as isize) / pointee_size as isize
}

struct Holder<T: Scalar> {
	start: *const T
}

unsafe impl<T: Scalar> Send for Holder<T> {}

unsafe impl<T: Scalar> Sync for Holder<T> {}

impl<T: Scalar + Float> Upfirdn<T> {
	pub fn new(p: usize, q: usize, coefs: RowVec<T, Dynamic>) -> Self {
		let coefs_per_phase = functions::quotient_ceil(coefs.size(), p);
		let mut coefs_t = ContainerRM::zeros(D!(p), D!(coefs_per_phase));

		for r in 0..p {
			for c in 0..coefs_per_phase {
				if c * p + r < coefs.size() {
					coefs_t[(coefs_per_phase - 1 - c) + r * coefs_per_phase] = coefs[c * p + r]
				}
			}
		}

		Self { p, q, coefs_t, coefs_per_phase }
	}

	pub fn out_count(&self, in_count: usize) -> usize {
		functions::quotient_ceil(in_count * self.p, self.q)
	}

	pub fn coefs_per_phase(&self) -> usize { self.coefs_per_phase }

	pub fn apply_parallel<D, S>(&self, input: &S) -> RowVec<T, Dynamic>
		where D: Dim, S: RowVecStorage<T, D>
	{
		let mut ret = rvec_zeros!(Dynamic::new(self.out_count(input.col_count())));

		// TODO: this whole thing is built around shifting pointers. How to keep is safe without speed penalty?
		unsafe {
			let start = input.get_ptr_unchecked(0, 0);
			let holder = Holder { start };
			//let mut cursor = start; // Equal to (i * q) / p
			//let mut phase = 0; // Equal to (i * q) % p
			let window_size_max = self.coefs_per_phase as isize - 1;

			let it_count = (input.size() * self.p) / self.q;
			ret.as_mut_slice().par_iter_mut().enumerate().for_each(|(i, out_cursor)| {
				let iq = i * self.q;
				let phase = iq % self.p;
				let cursor = holder.start.offset((iq / self.p) as isize);

				let mut acc = T::default();
				let window = min(offset_from(cursor, holder.start), window_size_max);
				let mut h = self.coefs_t.as_row_ptr(phase).offset(window_size_max - window);
				let mut cursor_before = cursor.offset(-window);

				for _ in 0..window + 1 {
					acc += get_n_advance!(cursor_before) * get_n_advance!(h);
				}

				// acc = h_slice.iter().zip(x_slice.iter()).map(|(he, ce)| *he * *ce).sum()
				*out_cursor = acc;
			});
		}

		ret
	}

	pub fn apply<D, S>(&self, input: &S) -> RowVec<T, Dynamic>
		where D: Dim, S: RowVecStorage<T, D>
	{
		let mut ret = rvec_zeros!(Dynamic::new(self.out_count(input.col_count())));

		// TODO: this whole thing is built around shifting pointers. How to keep is safe without speed penalty?
		unsafe {
			let start = input.get_ptr_unchecked(0, 0);
			let holder = Holder { start };
			let mut cursor = start; // Equal to (i * q) / p
			let mut phase = 0; // Equal to (i * q) % p
			let window_size = self.coefs_per_phase as isize - 1;

			for (i, out_cursor) in ret.as_mut_slice().iter_mut().enumerate() {
				let mut acc = T::default();
				let window = min(offset_from(cursor, start), window_size);
				let mut h = self.coefs_t.as_row_ptr(phase).offset(window_size - window);
				let mut cursor_before = cursor.offset(-window);

				for _ in 0..window + 1 {
					acc += get_n_advance!(cursor_before) * get_n_advance!(h);
				}

				*out_cursor = acc;

				cursor = cursor.offset(((phase + self.q) / self.p) as isize);
				phase = (phase + self.q) % self.p;
			}
		}

		ret
	}
}

/// Polyphrase FIR resampler [source](https://sourceforge.net/motorola/upfirdn/home/Home/)
/// Implementation is modified by me to support parallelism. (seems to yield a ~3x improvement (16 cores). Probably mem bound)
/// # Arguments
/// * `s` - input siganal
/// * `p` - upsampling rate
/// * `q` - downsampling rate
/// * `coefs` - FIR filter
pub fn upfirdn<T, D, S>(s: &S, p: usize, q: usize, coefs: RowVec<T, Dynamic>)
	-> RowVec<T, Dynamic>
	where T: Scalar + Float, D: Dim, S: RowVecStorage<T, D>
{
	let m = Upfirdn::new(p, q, coefs);
	let padding = rvec_zeros!(D!(m.coefs_per_phase()); T);
	let sa = join_cols!(s, padding);
	m.apply_parallel(&sa)
}