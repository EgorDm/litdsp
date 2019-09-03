use litcontainers::*;
use fftw::plan::*;
use fftw::types::Flag;

pub struct STFTFeed<P, W>
	where P: R2CPlan, P::Real: Element, P::Complex: Element,
	      W: Dim, W: Dim + DimDiv<U2>, <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	window_dim: W,
	plan: P,
}

impl<P, W> STFTFeed<P, W>
	where P: R2CPlan, P::Real: Element, P::Complex: Element,
	      W: Dim, W: Dim + DimDiv<U2>, <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	pub fn new(window_dim: W) -> Self {
		let window_size = window_dim.value();
		Self { window_dim, plan: P::aligned(&[window_size], Flag::Estimate).unwrap() }
	}

	pub fn in_dim(&self) -> W { self.window_dim.clone() }

	pub fn in_len(&self) -> usize { self.in_dim().value() }

	pub fn out_dim(&self) -> <<W as DimDiv<U2>>::Output as DimAdd<U1>>::Output { self.window_dim.div(U2).add(U1) }

	pub fn out_len(&self) -> usize { self.out_dim().value() }

	pub fn next<I: Storage<P::Real>, O: StorageMut<P::Complex>>(&mut self, input: &I, output: &mut O) {
		debug_assert_eq!(input.len(), self.in_len());
		debug_assert_eq!(output.len(), self.out_len());
		let input_slice = input.as_slice() as *const _; // Input does not actually change but must be provided as mut ref
		self.plan.r2c(unsafe { &mut *(input_slice as *mut _) }, output.as_slice_mut()).unwrap();
	}
}

pub type STFTF64Feed<W> = STFTFeed<R2CPlan64, W>;
pub type STFTF32Feed<W> = STFTFeed<R2CPlan32, W>;

pub struct ISTFTFeed<P, W>
	where P: C2RPlan, P::Real: Element, P::Complex: Element,
	      W: Dim, W: Dim + DimDiv<U2>, <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	window_dim: W,
	plan: P,
}

impl<P, W> ISTFTFeed<P, W>
	where P: C2RPlan, P::Real: Element, P::Complex: Element,
	      W: Dim, W: Dim + DimDiv<U2>, <W as DimDiv<U2>>::Output: DimAdd<U1>
{
	pub fn new(window_dim: W) -> Self {
		let window_size = window_dim.value();
		Self { window_dim, plan: P::aligned(&[window_size], Flag::Estimate).unwrap() }
	}

	pub fn in_dim(&self) -> <<W as DimDiv<U2>>::Output as DimAdd<U1>>::Output { self.window_dim.div(U2).add(U1) }

	pub fn in_len(&self) -> usize { self.in_dim().value() }

	pub fn out_dim(&self) -> W { self.window_dim.clone() }

	pub fn out_len(&self) -> usize { self.out_dim().value() }

	pub fn next<I: Storage<P::Complex>, O: StorageMut<P::Real>>(&mut self, input: &I, output: &mut O) {
		debug_assert_eq!(input.len(), self.in_len());
		debug_assert_eq!(output.len(), self.out_len());
		let input_slice = input.as_slice() as *const _; // Input does not actually change but must be provided as mut ref
		self.plan.c2r(unsafe { &mut *(input_slice as *mut _) }, output.as_slice_mut()).unwrap();
	}
}

pub type ISTFTF64Feed<W> = ISTFTFeed<C2RPlan64, W>;
pub type ISTFTF32Feed<W> = ISTFTFeed<C2RPlan32, W>;