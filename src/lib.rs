pub mod windowed_iter;
pub mod stft;
pub mod window;

pub use windowed_iter::*;
pub use stft::*;
pub use window::*;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
