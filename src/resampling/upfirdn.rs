use litcontainers::*;
use crate::functions;
use std::cmp::{max, min};
use num_traits::real::Real;

struct Upfirdn {
	p: usize,
	q: usize,
	coefs_t: ContainerRM<f64, Dynamic, Dynamic>,
	coefs_per_phase: usize
}

macro_rules! get_n_advance {
	($ptr: ident) => ({
		let ret = &*$ptr;
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

impl Upfirdn {
	pub fn new(p: usize, q: usize, coefs: RowVec<f64, Dynamic>) -> Self {
		let coefs_per_phase = functions::quotient_ceil(coefs.size(), p);
		let padded_coef_count = coefs_per_phase * p;
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

	pub fn apply<D: Dim>(&self, input: &RowVec<f64, D>) -> RowVec<f64, Dynamic> {
		let mut ret = rvec_zeros!(Dynamic::new(self.out_count(input.col_count())));

		println!("{}", &input);


		// TODO: this whole thing is built around shifting pointers. How to keep is safe without speed penalty?
		unsafe {
			let start = input.get_ptr_unchecked(0, 0);
			let end = start.offset(input.size() as isize);
			let mut cursor = start;
			let mut out_cursor = ret.get_mut_ptr_unchecked(0, 0);
			let mut phase = 0;
			let window_size = self.coefs_per_phase as isize - 1;

			while cursor < end {
				let mut acc = 0.;
				let offset = min(offset_from(cursor, start), window_size);
				let mut h = self.coefs_t.as_row_ptr(phase).offset(window_size - offset);
				let mut cursor_before = cursor.offset(-offset);
				let test2 = self.coefs_t.calc_index(phase, 0);

				while cursor_before <= cursor {
					let a = *cursor_before;
					let b = *h;
					acc += get_n_advance!(cursor_before) * get_n_advance!(h);
				}

				*get_n_advance_mut(&mut out_cursor) = acc;

				cursor = cursor.offset(((phase + self.q) / self.p) as isize);
				phase = (phase + self.q) % self.p;
			}
		}

		ret
	}
}

pub fn upfirdn<D: Dim>(s: &RowVec<f64, D>, p: usize, q: usize, coefs: RowVec<f64, Dynamic>)
	-> RowVec<f64, Dynamic> {
	let m = Upfirdn::new(p, q, coefs);
	m.apply(s)
}