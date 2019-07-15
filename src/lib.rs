pub mod windowed_iter;
pub mod stft;
pub mod window;
pub mod wave;
pub mod functions;

pub use windowed_iter::*;
pub use stft::*;
pub use window::*;
pub use wave::*;
pub use functions::*;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
