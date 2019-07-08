pub mod windowed_iter;
pub mod window_shifter;
pub mod stft;



#[cfg(test)]
mod tests {
	use litcontainers::*;
	use crate::window_shifter::{ColWindower, Windower};
	use crate::windowed_iter::*;

	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn window_shift() {
		let data = ContainerRM::from_vec(U2, Dynamic::new(10), (0..20).map(|x| x as f32).collect());
		let slice = data.slice_rows(1);

		let mut windower = WindowedColIter::new_padded(&slice, U2, U1, 1, 1);

		loop {
			match windower.next_window() {
				Some(s) => {
					dbg!(s.as_slice());
				},
				None => break
			}
		}
	}
}
