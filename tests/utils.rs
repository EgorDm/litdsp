use litcontainers::*;
use litdsp::*;

#[test]
fn generate_pulse() {
	let w = utils::generate_pulse(1., U20, 0, 20);
	dbg!(w.as_slice());
}

#[test]
fn stft() {
	let s = utils::generate_pulse(1., U40, 0, 6);
	let w = ContainerRM::from_value(U1, U12, 1.);
	let S = stft::calculate_stft(s, w, U6);
	let fs = S.norm();

	let i = 0;

}